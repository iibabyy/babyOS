use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::io::vga::{
    buffer::{Buffer, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH},
    color_code::ColorCode
};

lazy_static! {
    pub static ref GLOBAL_WRITER: Mutex<Writer> = Mutex::new(
        Writer {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::default(),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
        }
    );
}

pub struct Writer {
    pub column_position: usize,
    pub row_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }

        Ok(())
    }
}

impl Writer {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\t' => self.column_position += 4 - (self.column_position % 4),

            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let byte_to_write = match byte {
                    0x20..=0x7e => byte,
                    _ => 0xfe,
                };

                self.buffer.write(
                    self.row_position,
                    self.column_position,
                    ScreenChar::new(byte_to_write, self.color_code),
                );

                self.column_position += 1;
            },
        }
    }

    /// Moves to the next line, scrolling if necessary
    fn new_line(&mut self) {
        self.column_position = 0;
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            self.scroll_down();
        }
    }

    fn scroll_down(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.read(row, col);
                self.buffer.write(row - 1, col, char);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
    }

    /// Clears a single row with blank characters
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::new(b' ', self.color_code);

        for col in 0..BUFFER_WIDTH {
            self.buffer.write(row, col, blank);
        }
    }
}
