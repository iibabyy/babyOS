/// Magic number passed by the bootloader
pub const GRUB_MULTIBOOT_MAGIC: u32 = 0x2badb002;

/// Information passed by the bootloader
#[repr(C)]
#[expect(missing_docs)]
pub struct MultibootInfo {
	pub flags: u32,
	pub mem_lower: u32,
	pub mem_upper: u32,
	pub boot_device: u32,
	pub cmdline: u32,
	pub mods_count: u32,
	pub mods_addr: u32,
	pub syms: [u32; 4],

	// we only care about these (and the 6th bit of .flags)
	pub mmap_length: u32,
	pub mmap_addr: u32,
}

/// entries given by the bootloader at [MultibootInfo::mmap_addr]
#[repr(C, packed)]
pub struct MemoryMapEntry {
	/// entry.size doesn't include itself
	pub size: u32,

	pub base_addr_low: u32,
	pub base_addr_high: u32, // for 64-bit addresses only, but we are in 32-bit so it should be 0
	pub length_low: u32,
	pub length_high: u32,

	/// 1 = usable
	pub region_type: u32,
}

impl MemoryMapEntry {
	/// Returns false if the base address has more than 32 bits
	pub fn is_32_bit_address(&self) -> bool {
		self.base_addr_high == 0
	}

	/// Returns true if the region type is usable (equals 1)
	pub fn is_usable(&self) -> bool {
		self.region_type == 1
	}
}
