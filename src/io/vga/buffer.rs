use volatile::Volatile;

use crate::io::vga::color::ColorCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub(super) struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

pub(super) const BUFFER_HEIGHT: usize = 25;
pub(super) const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub(super) struct Buffer {
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
