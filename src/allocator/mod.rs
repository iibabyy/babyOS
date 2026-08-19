mod linked_list;

use core::alloc::GlobalAlloc;

use spin::Mutex;

use crate::allocator::linked_list::{
	LinkedListAllocator,
	ListNode,
};
use crate::paging::page_directory::PageDirectory;
use crate::paging::pmm::{
	FRAME_SIZE,
	kmalloc,
};

#[global_allocator]
pub static VIRTUAL_ALLOCATOR: LockedHeap = LockedHeap::empty();

/// We choose this address to be:
///  - far from the kernel code (0x00100000, cf. [crate::paging::kernel_start])
///  - far from the VGA buffer (0xB8000, cf. [crate::vga::VGA_BUFFER_ADDRESS])
///  - far below the recursive page tables (0xFFC00000, cf.
///    [crate::paging::page_directory::PAGE_TABLES_ADDRESS])
///
/// By doing this, we can grow upward without colliding with used memory spaces
const HEAP_START_ADDRESS: usize = 0xd000_0000;
const HEAP_SIZE: usize = 128 * 1024; // 100 MB

#[repr(transparent)]
pub struct LockedHeap(pub Mutex<LinkedListAllocator>);

impl LockedHeap {
	pub const fn empty() -> Self {
		Self(Mutex::new(LinkedListAllocator::new()))
	}
}

unsafe impl GlobalAlloc for LockedHeap {
	unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
		let mut allocator = self.0.lock();

		let ptr = match allocator.take_free_region(layout) {
			Some(ptr) => {
				// println!("allocated {ptr:p}: {:#?}", core::ptr::metadata(ptr));
				ptr
			}
			None => core::ptr::null_mut(),
		};
		ptr
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
		let size = layout.size().max(size_of::<ListNode>());

		let mut allocator = self.0.lock();
		allocator.add_free_region(ptr as usize, size);
	}
}

pub fn init_virtual_allocator() {
	debug_assert_eq!(HEAP_START_ADDRESS % FRAME_SIZE, 0, "Heap start must be page-aligned");

	let num_frames = HEAP_SIZE / FRAME_SIZE;

	for i in 0..num_frames {
		let physical_frame_addr = kmalloc().expect("Out of memory");
		let current_vaddr = HEAP_START_ADDRESS + i * FRAME_SIZE;

		// Safety:
		// 	 `physical_frame_addr` is valid
		unsafe {
			PageDirectory::map_page(current_vaddr as u32, physical_frame_addr, false, true);
		}
	}

	unsafe {
		// Safety: both args are valid
		VIRTUAL_ALLOCATOR.0.lock().init(HEAP_START_ADDRESS, num_frames * FRAME_SIZE);
	}
}
