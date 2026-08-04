mod multiboot;
mod pages;
mod pmm;

use self::multiboot::MemoryMapEntry;
pub use self::multiboot::{
	GRUB_MULTIBOOT_MAGIC,
	MultibootInfo,
};
use self::pmm::GLOBAL_ALLOCATOR;
pub use self::pmm::{
	pmm_allocate_frame,
	pmm_deallocate_frame,
};

// kernel start and end addresses from link.ld (tools/build/link.ld)
unsafe extern "C" {
	static kernel_start: u32;
	static kernel_end: u32;
}

/// Reads the bootloader memory map described by `mb_info`
/// and free the unused memory spaces in the [GLOBAL_ALLOCATOR]
///
/// Note: the bootloader send this map when loading the kernel to describe what
/// memory spaces are used (free) or not
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

		// if the entry is usable (free), we mark it as free in our allocator
		if entry.is_usable() && entry.is_32_bit_address() {
			let start = entry.base_addr_low as usize;
			let end = start.saturating_add(entry.length_low as usize);
			allocator.free_region(start, end);
		}

		// entry.size don't count itself, so we add it (u32 = 4 bytes)
		// Note: we can't use size_of(MemoryMapEntry) because some computers send more
		// infos than what we capture
		current_addr += entry.size + 4;
	}

	// reserve the kernel memory space
	let kernel_start_address = core::ptr::addr_of!(kernel_start);
	let kernel_end_address = core::ptr::addr_of!(kernel_end);
	allocator.reserve_region(kernel_start_address as usize, kernel_end_address as usize);

	// reserve the hardware memory space (VGA, BIOS, etc...)
	allocator.reserve_region(0x0, 0x100000);
}
