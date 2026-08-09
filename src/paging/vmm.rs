use core::sync::atomic::AtomicUsize;

use crate::paging::alloc::{
	kfree,
	kmalloc,
};
use crate::paging::page_directory::PageDirectory;
use crate::paging::pmm::FRAME_SIZE;

/// Start at 0x0040_0000, just after the 4MB indentity-mapped kernel space;
static mut NEXT_FREE_VIRTUAL_ADDRESS: AtomicUsize = AtomicUsize::new(0x0040_0000);

/// # Safety
///  - Paging must be turned on
///  - `setup_directory_backdoor` must have been called
pub unsafe fn allocate_virtual_frame(size: usize) -> *mut u8 {
	if size == 0 {
		return core::ptr::null_mut();
	}

	// we add size_of(usize) because we will store the allocation size at the beginning of the block
	let total_size = size + size_of::<usize>();
	// calculate how many 4KB page frames we will need
	// div_ceil() round the result up
	let num_frames = total_size.div_ceil(FRAME_SIZE);

	#[allow(static_mut_refs)]
	let start_virtual_addr = unsafe {
		NEXT_FREE_VIRTUAL_ADDRESS
			.fetch_add(num_frames * FRAME_SIZE, core::sync::atomic::Ordering::Release)
	};

	for i in 0..num_frames {
		let physical_frame_addr = kmalloc().expect("Out of memory");
		let current_vaddr = start_virtual_addr + i * FRAME_SIZE;

		// Safety:
		// 	- `physical_frame_addr` is valid
		unsafe {
			PageDirectory::map_page(current_vaddr as u32, physical_frame_addr, true, true);
		}
	}

	// store the size of the allocation
	unsafe {
		core::ptr::write(start_virtual_addr as *mut usize, size);
	}

	(start_virtual_addr + size_of::<usize>()) as *mut u8
}

/// # Safety
///  - Paging must be turned on
///  - `setup_directory_backdoor` must have been called
pub unsafe fn deallocate_virtual_frame(virtual_addr: *mut u8) {
	let alloc_start_addr = virtual_addr as usize - size_of::<usize>();
	let alloc_size = unsafe { *(alloc_start_addr as *mut usize) };

	let total_size = alloc_size + size_of::<usize>();
	let num_frames = total_size.div_ceil(FRAME_SIZE);

	for i in 0..num_frames {
		let current_vaddr = alloc_start_addr + i * FRAME_SIZE;

		let physical_addr = unsafe { PageDirectory::unmap_page(current_vaddr as u32) };

		match physical_addr {
			Some(addr) => kfree(addr),
			None => panic!("KERNEL BUG: Invalid Free detected on virtual address {:#x}", current_vaddr)
		}
	}
}

/// # Safety
///  - Paging must be turned on
///  - `setup_directory_backdoor` must have been called
pub unsafe fn get_vmalloc_size(virtual_addr: *mut u8) -> usize {
	let alloc_start_addr = virtual_addr as usize - size_of::<usize>();
	unsafe { *(alloc_start_addr as *mut usize) }
}