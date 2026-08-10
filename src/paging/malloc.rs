use crate::paging::pmm::GLOBAL_ALLOCATOR;
use crate::paging::vmm;

/// Allocates a physical page frame of 4096 bytes
///
/// Returns None if there is no free memory available
///
/// Note: this helper locks the [GLOBAL_ALLOCATOR], so it should not be used
/// after manually locking the allocator
pub fn kmalloc() -> Option<u32> {
	GLOBAL_ALLOCATOR.lock().allocate_physical_frame()
}

/// Deallocates a physical page frame
///
/// `physical_address` should be the first address of a physical page frame
///
/// Note: this helper locks the [GLOBAL_ALLOCATOR], so it should not be used
/// after manually locking the allocator
pub fn kfree(physical_address: u32) {
	GLOBAL_ALLOCATOR.lock().deallocate_physical_frame(physical_address);
}

/// Allocate a contiguous memory block in virtual address space
pub fn vmalloc(size: usize) -> *mut u8 {
	// Safety: we assume that paging has been setup so we don't make this function unsafe
	unsafe { vmm::allocate_virtual_frame(size) }
}

/// Free a memory block allocated by [vmalloc]
/// 
/// # Note
/// 
/// `vaddr` must point to memory allocated by [vmalloc]
pub fn vfree(vaddr: *mut u8) {
	unsafe { vmm::deallocate_virtual_frame(vaddr) };
}

/// Return the size of a memory bock allocated by [vmalloc]
/// 
/// # Note
/// `vaddr` must point to memory block allocated by [vmalloc]
pub fn vmalloc_size(vaddr: *mut u8) -> usize {
	// Safety: we assume that paging has been setup so we don't make this function unsafe
	unsafe { vmm::get_vmalloc_size(vaddr) }
}