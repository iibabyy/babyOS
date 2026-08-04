use crate::vga::ColorCode;
use volatile::Volatile;

pub const VGA_BUFFER_HEIGHT: usize = 25;
pub const VGA_BUFFER_WIDTH: usize = 80;
pub const VGA_BUFFER_ADDRESS: *mut u16 = 0xB8000 as *mut u16;

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}

impl Buffer {
    pub fn write(&mut self, row: usize, col: usize, char: ScreenChar) {
        self.chars[row][col].write(char);
    }

    pub fn read(&self, row: usize, col: usize) -> ScreenChar {
        self.chars[row][col].read()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub byte: u8,
    pub color_code: ColorCode,
}

impl ScreenChar {
    pub const fn new(byte: u8, color_code: ColorCode) -> Self {
        Self { byte, color_code }
    }
}
