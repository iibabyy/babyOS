//! VGA Text Buffer Writer Implementation
//!
//! Provides the core writing functionality for VGA text output

use core::fmt::Write;

use crate::io::vga::{buffer::*, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH};

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

impl Write for Writer {
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