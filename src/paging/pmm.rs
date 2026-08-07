use spin::Mutex;

pub static GLOBAL_ALLOCATOR: Mutex<FrameAllocator> =
	Mutex::new(FrameAllocator::new_with_every_frame_reserved());

pub(crate) const FRAME_SIZE: usize = 4096;
pub(crate) const TOTAL_FRAMES: usize = 1_048_576; // 4GB / 4096
const BITMAP_LENGTH: usize = TOTAL_FRAMES / 32; // 32,768 u32 blocks

pub struct FrameAllocator {
	/// 0 = free, 1 = used.
	bitmap: [u32; BITMAP_LENGTH],
}

impl FrameAllocator {
	/// Creates a [FrameAllocator] with every frame marked as used
	pub const fn new_with_every_frame_reserved() -> Self {
		// u32::MAX is 0xFFFFFFFF (every bits set to 1)
		Self {
			bitmap: [u32::MAX; BITMAP_LENGTH],
		}
	}

	/// Allocates a 4096 bytes frame
	///
	/// Returns None if there is no free memory available
	pub fn allocate_frame(&mut self) -> Option<u32> {
		// find the first 32-bit block that isn't completely full of 1s
		for (index, &block) in self.bitmap.iter().enumerate() {
			if block != u32::MAX {
				// trick to find the index of the first '0' bit
				let bit_index = (!block).trailing_zeros();

				let frame_number = (index as u32 * 32) + bit_index;

				self.set_frame_used(frame_number);

				let physical_address = frame_number * FRAME_SIZE as u32;
				return Some(physical_address);
			}
		}

		None
	}

	/// Deallocates a frame
	///
	/// `physical_address` should be the first address of a frame
	pub fn deallocate_frame(&mut self, physical_address: u32) {
		let frame_number = physical_address / FRAME_SIZE as u32;
		self.set_frame_free(frame_number);
	}

	/// Calls [FrameAllocator::set_frame_used] as many times as needed on a region in memory
	pub fn reserve_region(&mut self, start_address: u32, end_address: u32) {
		// We round DOWN the start, and round UP the end to ensure we only reserve safe frames
		// e.g. if we receive 3000 and 10000, we reserve 0-12288 instead of 4096-8192
		let start_frame = start_address / FRAME_SIZE as u32;
		let end_frame = end_address.div_ceil(FRAME_SIZE as u32);

		for frame in start_frame..end_frame {
			self.set_frame_used(frame);
		}
	}

	/// Calls [FrameAllocator::set_frame_free] as many times as needed on a region in memory
	pub fn free_region(&mut self, start_address: u32, end_address: u32) {
		// We round UP the start, and round DOWN the end to ensure we only free safe frames
		// e.g. if we receive 3000 and 10000, we only free 4096-8192 instead of 0-12288
		let start_frame = start_address.div_ceil(FRAME_SIZE as u32);
		let end_frame = end_address / FRAME_SIZE as u32;

		for frame in start_frame..end_frame {
			self.set_frame_free(frame);
		}
	}

	pub fn set_frame_used(&mut self, frame_number: u32) {
		let index = frame_number / 32;
		let bit = frame_number % 32;

		// Bitwise OR assigns 1 to the exact bit without changing the rest
		self.bitmap[index as usize] |= 1 << bit;
	}

	pub fn set_frame_free(&mut self, frame_number: u32) {
		let index = frame_number / 32;
		let bit = frame_number % 32;

		// Bitwise AND NOT assigns 0 to the exact bit without changing the rest
		self.bitmap[index as usize] &= !(1 << bit);
	}
}
