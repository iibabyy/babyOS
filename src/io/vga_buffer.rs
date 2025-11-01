//! VGA Text Buffer Implementation
//! 
//! Handles low-level text output to the VGA buffer at 0xb8000

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

// ================================
// Constants
// ================================

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

// ================================
// Global Writer Instance
// ================================

lazy_static! {
	/// Global writer instance for safe console output
	/// Provides thread-safe access to the VGA text buffer
	pub static ref WRITER: Mutex<Writer> = Mutex::new(
		Writer {
			column_position: 0,
			row_position: 0,
			color_code: ColorCode::new(Color::White, Color::Black),
			buffer: unsafe { &mut *(0xb8000 as *mut self::Buffer) }
		}
	);
}

// ================================
// Color Definitions
// ================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// ================================
// Screen Buffer Structures
// ================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// ================================
// Writer Implementation
// ================================

pub struct Writer {
    pub column_position: usize,
    pub row_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    /// Writes a single byte to the buffer
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\t' => {
                // Tab character - advance to next tab stop (every 4 spaces)
                self.column_position += 4 - (self.column_position % 4);
            },
            b'\n' => self.new_line(),  // New line
            byte => {
                // Regular character
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                let color_code = self.color_code;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                self.column_position += 1;
            }
        }
    }

    /// Writes a string to the buffer
    pub fn write_string(&mut self, str: &str) {
        for byte in str.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\t' | b'\n' => self.write_byte(byte),
                // non printable ASCII - use solid block character
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Moves to the next line, scrolling if necessary
    fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            // Scroll: move all lines up by one, clear the last line
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let char = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(char);
                }
            }

            self.clear_row(BUFFER_HEIGHT - 1);
        }

        self.column_position = 0;
    }

    /// Clears a single row with blank characters
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// ================================
// Public Interface Functions
// ================================

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    crate::io::vga_buffer::WRITER.lock().write_fmt(args).unwrap();
}
