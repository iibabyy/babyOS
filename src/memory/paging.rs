use core::ops::{
	Index,
	IndexMut,
};

use modular_bitfield::specifiers::{
	B2,
	B4,
	B20,
};
use modular_bitfield::{
	Specifier,
	bitfield,
};

use crate::memory::pmm::FRAME_SIZE;
use crate::pmm_allocate_frame;

/// Contains 1023 pointers to [PageTable]s
/// The 1024-th pointer is a pointer to a [PageDirectory] (backdoor)
/// (cf. [PageDirectory::setup_directory_backdoor] documentation)
#[repr(C, align(4096))]
pub struct PageDirectory {
	/// Array of [PageTable] pointers
	pub table_pointers: [PagePointer; 1023],

	/// Created only for type safety,
	/// it really just is the last pointer of the array
	pub backdoor: PagePointer,
}

impl PageDirectory {
	pub const fn zeroed() -> Self {
		PageDirectory {
			table_pointers: [PagePointer::zeroed(); 1023],
			backdoor: PagePointer::zeroed(),
		}
	}

	/// Connects a virtual address to a physical address
	///
	/// This function assumes that paging is enabled and `self.backdoor` is initialized.
	///  
	/// # Safety:
	///  - Paging must be turned on
	///  - `setup_directory_backdoor` must have been called
	///  - `physical_addr` must point to a valid, allocated physical address
	pub(crate) unsafe fn map_page(virtual_addr: u32, physical_addr: u32, flags: PageEntryFlags) {
		// cut the virtual address in the following way:
		//  - 10 first bits: directory offset
		//  - 10 middle bits: table offset
		//  - 12 last bits: page offset (used by the CPU)
		let dir_offset = (virtual_addr >> 22) as usize; // Top 10 bits
		let table_offset = ((virtual_addr >> 12) & 0x3ff) as usize; // Middle 10 bits

		let directory = unsafe { Self::backdoor_directory() };

		// if there is no table at dir_offset, create one
		if !directory[dir_offset].flags().is_present() {
			unsafe { directory.allocate_table_at(dir_offset) };
		}

		let backdoor_table = unsafe { PageDirectory::backdoor_table_at(dir_offset) };

		backdoor_table[table_offset].set(physical_addr, flags);
	}

	/// Allocate a [PageTable] and set a pointer to it at `backdoor_directory()[dir_index]`
	///
	/// # Safety:
	///  - Paging must be turned on
	///  - [PageDirectory::setup_directory_backdoor] must have been called
	///  - `self` must have been created using [PageDirectory::backdoor_directory]
	unsafe fn allocate_table_at(&mut self, dir_index: usize) {
		let new_physical_frame =
			pmm_allocate_frame().expect("Out of memory when creating page table!");

		// We can't directly use `new_physical_frame` to clear the memory,
		// as the CPU treats every address as a virtual addresses.

		let flags = PageEntryFlags::new().with_is_present(true).with_is_writable(true);

		self.table_pointers[dir_index].set(new_physical_frame, flags);

		unsafe {
			core::ptr::write_bytes(new_physical_frame as *mut u8, 0, FRAME_SIZE);
		};
	}

	/// Creates a backdoor to a [PageDirectory] by setting its address into the last [PagePointer]
	/// of its own array.
	///
	/// When we turn paging on, the CPU can't access the [PageDirectory] by its physical address
	/// anymore, as it would treat this address as a virtual address, and thus translate it into a
	/// completely different physical address.
	///
	/// To solve this, we set the pointer in the last [PagePointer] of the [PageDirectory] to its
	/// own physical address.
	///
	/// When we then access the `0xFFFFF000` virtual address, the CPU will access the
	/// [PageDirectory] by doing the following:
	///  - First 10 bits: 1023: goes to the physical address pointed by the last [PagePointer] of
	///    the [PageDirectory], which is the [PageDirectory] physical address.
	///  - Middle 10 bits: 1023: again, goes to the [PageDirectory] physical address.
	///  - Last 12 bits: 0: stays on the first byte of the [PageDirectory] physical address.
	///
	/// Voilà :) The CPU will return the original [PageDirectory], if we ask for the right amount of bytes.
	///
	/// You can read [https://wiki.osdev.org/X86_Paging] for a better understanding.
	///
	/// # Notes
	///  - I only created [PageDirectory].backdoor for type safety, but,
	///    in memory, it's represented exactly as the 1024-th [PagePointer] of the pointer array
	pub(crate) fn setup_directory_backdoor(&mut self) {
		let self_addr = &raw const *self;
		let flags = PageEntryFlags::new().with_is_present(true).with_is_writable(true);

		self.backdoor.set(self_addr as u32, flags);
	}

	/// Creates a reference to the 'original' [PageDirectory]
	///
	/// When paging is turned on, we can't use self.tables[i] (address_of(self) + offset)
	/// from a normal [PageDirectory], as CPU will treat that address as a virtual address,
	/// thus mapping it to a completely different physical address.
	///
	/// To do this, we must use the 0xFFFFF000 virtual address,
	/// that would map to the 'original' [PageDirectory]'s physical address by doing the following:
	///  - First 10 bits: 1023: goes to the physical address pointed by the last [PagePointer] of
	///    the [PageDirectory], which is the [PageDirectory] itself.
	///  - Middle 10 bits: 1023: again, goes to the [PageDirectory] physical address.
	///  - Last 12 bits: 0: stays on the first byte of the [PageDirectory] physical address.
	///
	/// We then simply take size_of([PageDirectory]), or size_of([PageEntry; 1024]) bytes
	/// from this address.
	///
	/// # Safety
	///  - Paging must be turned on
	///  - [PageDirectory::setup_directory_backdoor] must have been called
	const unsafe fn backdoor_directory() -> &'static mut PageDirectory {
		unsafe { &mut *(0xfffff000 as *mut PageDirectory) }
	}

	/// Creates a reference to a [PageTable] using a backdoor.
	///
	/// When paging is turned on, we cannot access a [PageTable] using the physical address
	/// stored in the [PageDirectory], as the CPU would treat it as a virtual address and crash.
	///
	/// To solve this, we use a backdoor starting at virtual address `0xFFC00000`.
	///
	/// When we access `0xFFC00000 + (index * 4096)`, the CPU maps it to the right [PageTable]
	/// physical address by doing the following:
	///  - First 10 bits: 1023: goes to the physical address pointed by the last [PagePointer] of
	///    the [PageDirectory], which is the [PageDirectory] itself.
	///  - Middle 10 bits: `index`: goes to the physical address pointed by the `index`-th
	///    [PagePointer] of the [PageDirectory], which is the `index`-th [PageTable]
	///  - Last 12 bits: 0: stays on the first byte of that [PageTable].
	///
	/// # Safety
	///  - Paging must be turned on.
	///  - [PageDirectory::setup_directory_backdoor] must have been called
	///  - The `index`-th [PagePointer] in [PageDirectory] must be allocated and marked as present
	const unsafe fn backdoor_table_at(index: usize) -> &'static mut PageTable {
		let table_address = 0xffc00000 + (index * 4096);
		unsafe { &mut *(table_address as *mut PageTable) }
	}
}

impl Index<usize> for PageDirectory {
	type Output = PagePointer;

	fn index(&self, index: usize) -> &Self::Output {
		&self.table_pointers[index]
	}
}

impl IndexMut<usize> for PageDirectory {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.table_pointers[index]
	}
}

#[repr(C, align(4096))]
pub struct PageTable {
	pub physical_page_pointers: [PagePointer; 1024],
}

impl PageTable {
	pub fn zeroed() -> Self {
		Self {
			physical_page_pointers: [PagePointer::zeroed(); 1024],
		}
	}
}

impl Index<usize> for PageTable {
	type Output = PagePointer;

	fn index(&self, index: usize) -> &Self::Output {
		&self.physical_page_pointers[index]
	}
}

impl IndexMut<usize> for PageTable {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.physical_page_pointers[index]
	}
}

/// Contains a pointer and some flags.
///
/// The pointer can point to diferrent things:
///  - When used in [PageTable]: all the pointers points to a physical page frame
///  - When used in [PageDirectory]:
/// 	 - First 1023 pointers: points to a [PageTable]
/// 	 - The 1024th pointer: points to a [PageDirectory]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PagePointer(RawPageEntry);

impl PagePointer {
	pub const fn zeroed() -> Self {
		Self(RawPageEntry::new())
	}

	/// SAFETY: `physical_address` must be 4KB aligned (divisible by 4096),
	/// so its bottom 12 bits are 0 (we will keep only the last 20 bits)
	pub unsafe fn new(flags: PageEntryFlags, physical_address: u32) -> Self {
		// TODO: We might have to handle the error without panicking
		debug_assert!(physical_address & 0xfff == 0, "physical_address must be 4KB aligned");

		// modular_bitfield will apply address << 12 (to keep only the first 20 bits)
		// so we apply address >> 12 to move the significant bits (last 20) to the first 20 bits
		let b20_physical_address = page_frame_address_to_b20(physical_address);

		Self(RawPageEntry::new().with_flags(flags).with_physical_address(b20_physical_address))
	}

	pub fn set(&mut self, physical_address: u32, flags: PageEntryFlags) {
		let b20_address = page_frame_address_to_b20(physical_address);

		self.0.set_flags(flags);
		self.0.set_physical_address(b20_address);
	}

	fn flags(&self) -> PageEntryFlags {
		self.0.flags()
	}

	fn physical_addr(&self) -> u32 {
		b20_to_page_frame_address(self.0.physical_address())
	}
}

#[bitfield(bits = 32)]
#[derive(Clone, Copy)]
pub struct RawPageEntry {
	flags: PageEntryFlags,

	/// last 20 bits of the physical address
	physical_address: B20,
}

#[bitfield(bits = 12)]
#[derive(Specifier)]
pub struct PageEntryFlags {
	/// Must be 1 for the [PagePointer] to be valid
	pub is_present: bool,

	/// 1 if page is writable (it's always readable)
	pub is_writable: bool,

	/// True if user (ring 3) can access
	pub is_user_space: bool,

	#[expect(unused)]
	reserved_1: B2,

	/// CPU sets this when page is read/written
	pub is_accessed: bool,

	/// CPU sets this when page is written to (page tables only).
	pub is_dirty: bool,

	/// For page directories only
	pub is_4_mb_pages: bool,

	#[expect(unused)]
	reserved_2: B4,
}

/// Takes a page frame address (4KB aligned), and moves the significant bits (last 20 bits)
/// to the first 20 bits so B20 doesn't erase them
///
/// We're returning u32 because B20 setters take u32
pub const fn page_frame_address_to_b20(addr: u32) -> u32 {
	addr >> 12
}

/// Takes a B20 and modifies it so the 20 significant bits (moved to the first 20 bits
/// by [page_frame_address_to_b20]) move back to the last 20 bits
///
/// We're taking u32 because B20 getters return u32
pub const fn b20_to_page_frame_address(b20: u32) -> u32 {
	b20 << 12
}
