//! Kernel memory initialization and management
//!
//! At the bottom, we're using a [FrameAllocator] (called a Physical Memory Manager, or PMM)
//! that holds a bitmap representing a 4GiB memory space divided into 4KiB physical page frames.
//! The i-th boolean in the bitmap tells us if the i-th page frame is used (true) or not (false).
//!
//! On top of that, we handle virtual memory with a [PageDirectory],
//! which holds 1023 [PagePointer]s to [PageTable]s,
//! which in turn hold 1024 [PagePointer]s to physical page frames.
//!
//! To map a virtual address to a physical address, the CPU divides the 32 bits in the following
//! way:
//!  - First 10 bits: directory offset (designates the i-th [PagePointer] (to a [PageTable]) in the
//!    [PageDirectory])
//!  - Middle 10 bits: table offset (designates the i-th [PagePointer] (to a physical page frame)
//!    in the [PageTable])
//!  - Last 12 bits: page offset (used by the CPU - represents an offset within the physical page
//!    frame itself)
//!
//! But one issue is still standing:
//!
//! When we turn paging on, CPU can't access the [PageDirectory] by its physical address anymore,
//! because it would treat it as a virtual address, and thus translate it into a completely
//! different physical address.
//!
//! To solve this, we set the [PageDirectory]'s own physical address into the last [PagePointer] of
//! its own array.
//!
//! Note: This is also why we only have 1023 [PagePointer]s to [PageTable]s in [PageDirectory].
//!
//! When we then access the `0xFFFFF000` virtual address, the CPU, using the translation
//! formula described above, will access the [PageDirectory] by doing the following:
//!  - First 10 bits: 1023: goes to the physical address pointed by the last [PagePointer] of the
//!    [PageDirectory], which is the [PageDirectory] physical address.
//!  - Middle 10 bits: 1023: again, goes to the [PageDirectory] physical address.
//!  - Last 12 bits: 0: stays on the first byte of the [PageDirectory] physical address.
//!
//! Voilà! The CPU will then return the [PageDirectory] (if we ask for size_of([PageDirectory])
//! bytes)
//!
//! #### Documentation
//!
//! You can read [https://wiki.osdev.org/Memory_management] and [https://wiki.osdev.org/X86_Paging]
//! for a better understanding.
//!
//! [FrameAllocator]: self::pmm::FrameAllocator
//! [PagePointer]: self::paging::PagePointer

pub mod alloc;
mod multiboot;
pub mod page_directory;
pub mod page_fault;
mod pmm;
mod vmm;

use self::alloc::kmalloc;
use self::multiboot::MemoryMapEntry;
pub use self::multiboot::{
	GRUB_MULTIBOOT_MAGIC,
	MultibootInfo,
};
use self::page_directory::{
	PageDirectory,
	PageEntryFlags,
	PageTable,
};
use self::pmm::{
	FRAME_SIZE,
	GLOBAL_ALLOCATOR,
};

// kernel start and end addresses from link.ld (tools/build/link.ld)
unsafe extern "C" {
	static kernel_start: u32;
	static kernel_end: u32;
}

/// Initializes the physical memory.
///
/// It reads the bootloader memory map described by `mb_info`
/// and marks the unused memory spaces in the [GLOBAL_ALLOCATOR] as free.
///
/// #### Note:
/// the bootloader sends this map when loading the kernel to describe what
/// memory spaces are usable (free) or not.
pub fn init_physical_memory(mb_info: &MultibootInfo) {
	// check if the memory map is present
	assert!((mb_info.flags & (1 << 6)) != 0, "no memory map provided by GRUB");

	// at this point, every address in the allocator is marked as used
	let mut allocator = GLOBAL_ALLOCATOR.lock();

	let mut current_addr = mb_info.mmap_addr;
	let end_addr = mb_info.mmap_addr + mb_info.mmap_length;

	// iterate over the memory map to know which addresses are usable (free)
	while current_addr < end_addr {
		// read the entry at current_addr
		let entry = unsafe { &*(current_addr as *const MemoryMapEntry) };

		// if the entry is usable (free) and valid (32 bits), we mark it as free in our allocator
		if entry.is_usable() && entry.is_32_bit_address() {
			let start = entry.base_addr_low;
			let end = start.saturating_add(entry.length_low);
			allocator.free_region(start, end);
		}

		// entry.size doesn't count itself, so we add it (u32 = 4 bytes)
		// Note: we can't use size_of(MemoryMapEntry) because some computers send more
		// information than what we capture in the structure.
		current_addr += entry.size + 4;
	}

	// reserve the kernel memory space
	let kernel_start_address = &raw const kernel_start;
	let kernel_end_address = &raw const kernel_end;
	allocator.reserve_region(kernel_start_address as u32, kernel_end_address as u32);

	// reserve the hardware memory space (VGA, BIOS, etc...)
	allocator.reserve_region(0x0, 0x100000);
}

/// Initializes the virtual memory.
///
/// It creates the [PageDirectory] and maps the first 4MB of physical memory to the same addresses
/// in virtual memory. After that, when the CPU looks for any virtual addr below 4MB, it will
/// be mapped to the exact same physical addr.
/// This is needed because when enabling paging, the CPU must find the kernel code (+vga screen,
/// etc...) at the same address.
///
/// It then creates a backdoor to the [PageDirectory] by setting its own address into the last
/// pointer of its own array, so we can access it after enabling paging.
///
/// # Safety
///  - physical memory must be initialized
pub unsafe fn init_virtual_memory() -> u32 {
	let directory_phys_address = kmalloc().expect("Out of memory");
	let directory = unsafe { &mut *(directory_phys_address as *mut PageDirectory) };

	// clear the page frame
	unsafe { core::ptr::write_bytes(directory_phys_address as *mut u8, 0, FRAME_SIZE) };

	//  This table will cover the first 4MB of RAM
	let table0_phys = kmalloc().expect("Out of memory");
	let table0 = unsafe { &mut *(table0_phys as *mut PageTable) };

	// fill table0 with physical addresses in the first 4MB
	// 4MB / FRAME_SIZE (4KiB) = 1024 page frames
	for i in 0..1024 {
		let addr = (i * FRAME_SIZE) as u32;
		let flags = PageEntryFlags::new().with_is_present(true).with_is_writable(true);

		table0[i].set(addr, flags);
	}

	// put table0 into the first pointer of the directory
	let table0_flags = PageEntryFlags::new().with_is_present(true).with_is_writable(true);
	directory[0].set(table0_phys, table0_flags);

	// setup the backdoor (cf. setup_directory_backdoor() documentation)
	directory.setup_directory_backdoor();

	directory_phys_address
}
