use crate::shared::outb;
use crate::vga::{
	VGA_BUFFER_HEIGHT,
	VGA_BUFFER_WIDTH,
};

/// SAFETY: `x` and `y` must be within the bounds of the VGA buffer
pub unsafe fn terminal_set_cursor(x: usize, y: usize) {
	if x >= VGA_BUFFER_WIDTH || y >= VGA_BUFFER_HEIGHT {
		return;
	}

	let pos: u16 = (y * VGA_BUFFER_WIDTH + x) as u16;

	// outb only writes 8 bits at a time
	unsafe {
		// Set the low byte first
		outb(0x3d4, 0x0f);
		outb(0x3d5, pos as u8);

		// Set the high byte
		outb(0x3d4, 0x0e);
		outb(0x3d5, (pos >> 8) as u8);
	}
}
