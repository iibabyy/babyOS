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
            infos: KernelWriterInfos {
                column_position: 0,
                row_position: 0,
                color_code: ColorCode::new(Color::White, Color::Black),
            },
            buffer: unsafe { &mut *(0xb8000 as *mut self::Buffer) }
        }
    );
}

// ================================
// KernelWriter Trait Definition
// ================================

pub struct KernelWriterInfos {
    pub column_position: usize,
    pub row_position: usize,
    pub color_code: ColorCode,
}

pub trait KernelWriter {
    fn infos(&mut self) -> &mut KernelWriterInfos;
    fn read(&self, row: usize, col: usize) -> ScreenChar;
    fn write(&mut self, row: usize, col: usize, byte: ScreenChar);

    #[inline(always)]
    fn write_byte(&mut self, byte: u8) {
        match byte {
            // Newline
            b'\n' => self.new_line(),

            // Tab character - advance to next tab stop (every 4 spaces)
            b'\t' => {
                let infos = self.infos();
                infos.column_position += 4 - (infos.column_position % 4);
            },

            // Regular character
            byte => {
                if self.infos().column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let infos = self.infos();

                let row = infos.row_position;
                let col = infos.column_position;

                let color_code = infos.color_code;

                let byte_to_write = if matches!(byte, 0x20..=0x7e | b'\t' | b'\n') {
                    byte
                } else {
                    0xfe
                };

                self.write(
                    row,
                    col,
                    ScreenChar {
                        ascii_character: byte_to_write,
                        color_code,
                    },
                );

                self.infos().column_position += 1;
            }
        }
    }

    #[inline(always)]
    fn write_string(&mut self, str: &str) {
        for byte in str.bytes() {
            self.write_byte(byte);
        }
    }

    /// Moves to the next line, scrolling if necessary
    #[inline(always)]
    fn new_line(&mut self) {
        let infos: &mut KernelWriterInfos = self.infos();

        infos.column_position = 0;
        if infos.row_position < BUFFER_HEIGHT - 1 {
            infos.row_position += 1;
        } else {
            // Scroll: move all lines up by one, clear the last line
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let char = self.read(row, col);
                    self.write(row - 1, col, char);
                }
            }

            self.clear_row(BUFFER_HEIGHT - 1);
        }
    }

    /// Clears a single row with blank characters
    #[inline(always)]
    fn clear_row(&mut self, row: usize) {
        let infos: &mut KernelWriterInfos = self.infos();

        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: infos.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.write(row, col, blank);
        }
    }
}

// ================================
// Writer Struct Definition
// ================================

pub struct Writer {
    pub infos: KernelWriterInfos,
    pub buffer: &'static mut Buffer,
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl KernelWriter for Writer {
    #[inline(always)]
    fn infos(&mut self) -> &mut KernelWriterInfos {
        &mut self.infos
    }

    #[inline(always)]
    fn read(&self, row: usize, col: usize) -> ScreenChar {
        self.buffer.chars[row][col].read()
    }

    #[inline(always)]
    fn write(&mut self, row: usize, col: usize, byte: ScreenChar) {
        self.buffer.chars[row][col].write(byte)
    }
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
// Public Interface Functions
// ================================

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    crate::io::vga_buffer::WRITER
        .lock()
        .write_fmt(args)
        .unwrap();
}
