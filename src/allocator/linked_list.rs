use core::alloc::Layout;

pub struct LinkedListAllocator {
	head: Option<*mut ListNode>,
}

unsafe impl Send for LinkedListAllocator {}

impl LinkedListAllocator {
	pub const fn new() -> Self {
		Self {
			head: None,
		}
	}

	/// Initialize the allocator
	///
	/// # Safety
	///  - `heap_start` and `heap_address` must design a valid and unused memory region
	pub unsafe fn init(&mut self, heap_start_vaddr: usize, heap_size: usize) {
		self.add_free_region(heap_start_vaddr, heap_size);
	}

	/// Add a memory region to the list
	pub fn add_free_region(&mut self, vaddr: usize, size: usize) {
		// make sure the region can hold a ListNode
		assert!(size >= align_of::<ListNode>());
		assert!(size >= size_of::<ListNode>());

		let mut node = ListNode::new(size);

		// put `self.head` into `node.next`
		node.next = self.head;

		// store `node` at address `addr`
		let node_addr = vaddr as *mut ListNode;
		let adjusted_addr = align_up(node_addr as usize, align_of::<ListNode>()) as *mut ListNode;
		unsafe { core::ptr::write(adjusted_addr, node) };

		self.head = Some(adjusted_addr)
	}

	pub fn take_free_region(&mut self, layout: core::alloc::Layout) -> Option<*mut u8> {
		// ensure the size is at least `size_of::<ListNode>()`
		let adjusted_size = layout.size().max(size_of::<ListNode>());
		let adjusted_layout = Layout::from_size_align(adjusted_size, layout.align()).unwrap();

		let mut current_option = self.head;
		let mut prev_option: Option<*mut ListNode> = None;

		while let Some(current_ptr) = current_option {
			let current = unsafe { &mut *current_ptr };

			// try to split
			if let Some(addr) = current.split(adjusted_layout) {
				// if there is no memory left in the node (perfect fit), remove it
				if current.size == 0 {
					self.remove_current_node(current, prev_option);
				}
				return Some(addr);
			}

			prev_option = Some(current_ptr);
			current_option = current.next;
		}

		// No space left
		None
	}

	fn remove_current_node(&mut self, current: &mut ListNode, prev_option: Option<*mut ListNode>) {
		if let Some(prev_ptr) = prev_option {
			unsafe { (*prev_ptr).next = current.next };
		} else {
			self.head = current.next;
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct ListNode {
	pub size: usize,
	pub next: Option<*mut ListNode>,
}

impl ListNode {
	pub const fn new(size: usize) -> Self {
		Self {
			size,
			next: None,
		}
	}

	pub fn start_vaddr(&self) -> usize {
		self as *const Self as usize
	}

	pub fn end_vaddr(&self) -> usize {
		self.start_vaddr() + self.size
	}

	fn split(&mut self, layout: Layout) -> Option<*mut u8> {
		let size = layout.size().max(size_of::<ListNode>());

		let alloc_end = self.end_vaddr();
		let alloc_start = align_down(alloc_end - size, layout.align());

		if alloc_start < self.start_vaddr() {
			return None;
		}

		let split_node_size = alloc_start - self.start_vaddr();

		if split_node_size < size_of::<ListNode>() {
			// TODO: return the whole block
			return None;
		}

		self.size = split_node_size;

		return Some(alloc_start as *mut u8);
	}
}

/// Align `addr` downwards to `align`
///
/// `align` must be a power of 2
pub const fn align_down(addr: usize, align: usize) -> usize {
	addr & !(align - 1)
}

/// Align `addr` upwards to `align`
///
/// `align` must be a power of 2
pub const fn align_up(addr: usize, align: usize) -> usize {
	align_down(addr + align - 1, align)
}
