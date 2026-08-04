pub const GRUB_MULTIBOOT_MAGIC: u32 = 0x2BADB002;

#[repr(C)]
pub struct MultibootInfo {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: [u32; 4],
    
    // we only care about them (and the 6th bit of .flags)
    pub mmap_length: u32,
    pub mmap_addr: u32,
}

/// entries given by the bootloader at [MultibootInfo].mmap_addr
#[repr(C, packed)]
pub struct MemoryMapEntry {
	/// entry.size don't count itself in it
	pub size: u32,

	pub base_addr_low: u32,
	pub base_addr_high: u32, // for 64 bit addresses only, but we are in 32 bit
	pub length_low: u32,
    pub length_high: u32,

	/// 1 = usable
    pub region_type: u32,
}

impl MemoryMapEntry {
	/// returns false if the base address have more than 32 bits
	pub fn is_32_bit_address(&self) -> bool {
		self.base_addr_high == 0
	}

	/// returns true if the region type is usable (equals to 1)
	pub fn is_usable(&self) -> bool {
		self.region_type == 1
	}
}
