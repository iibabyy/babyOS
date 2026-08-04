use modular_bitfield::{Specifier, bitfield, specifiers::{B6, B20}};

#[repr(C, align(4096))]
pub struct PageDirectory {
    /// Points to physical addresses of [PageTable]s
    pub entries: [PageEntry; 1024], 
}

impl PageDirectory {
    pub const fn zeroed() -> Self {
        PageDirectory {
            entries: [PageEntry::zeroed(); 1024],
        }
    }
}

#[repr(C, align(4096))]
pub struct PageTable {
	pub entries: [PageEntry; 1024]
}

impl PageTable {
	pub fn zeroed() -> Self {
		Self { entries: [PageEntry::zeroed(); 1024] }
	}
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PageEntry(RawPageEntry);

impl PageEntry {
	pub const fn zeroed() -> Self {
		Self(RawPageEntry::new())
	}

	/// SAFETY: `physical_address` must be 4KB aligned (divisible by 4096),
	/// so its bottom 12 bits are 0 (we keep only the last 20 bits)
	pub unsafe fn new(
		flags: PageEntryFlags,
		physical_address: u32
	) -> Self {
		debug_assert!(physical_address & 0xFFF == 0, "physical_address must be 4KB aligned");
		let address_last_20_bits = physical_address & 0xFFFFF;

		Self(
			RawPageEntry::new()
				.with_flags(flags)
				.with_physical_address(address_last_20_bits)
		)
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
	/// Must be 1 for the entry to be valid 
	is_present: bool,

	/// 1 if page is writable (it's always readable)
	is_writable: bool,

	/// true if user (ring 3) can access
	is_user_space: bool,

	/// CPU sets this when page is read/written
	is_accessed: bool,

	/// CPU sets this when page is written to (page tables only).
	is_dirty: bool,

	/// For page directories only
	is_4_mb_pages: bool,

	#[expect(unused)]
	padding: B6,
}


