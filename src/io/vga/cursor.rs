use x86::io::outb;

use crate::io::vga::buffer::{BUFFER_HEIGHT, BUFFER_WIDTH};

/// # Safety
/// The caller must ensure that `x` and `y` are within the bounds of the VGA buffer.
pub unsafe fn terminal_set_cursor(x: usize, y: usize) {
    if x >= BUFFER_WIDTH || y >= BUFFER_HEIGHT {
        return;
    }

    let pos: u16 = (y * BUFFER_WIDTH + x) as u16;

    // outb only writes 8 bits at a time
    unsafe {
        // Set the low byte first
        outb(0x3D4, 0x0F);
        outb(0x3D5, pos as u8);

        // Set the high byte
        outb(0x3D4, 0x0E);
        outb(0x3D5, (pos >> 8) as u8);
    }
}
