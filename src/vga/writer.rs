use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use crate::vga::{
    self, Buffer, ColorCode, ScreenChar, VGA_BUFFER_ADDRESS, VGA_BUFFER_HEIGHT, VGA_BUFFER_WIDTH,
};

lazy_static! {
    pub static ref GLOBAL_WRITER: Mutex<Writer> = {
        let buffer = unsafe { &mut *(VGA_BUFFER_ADDRESS as *mut Buffer) };

        // sets all bytes in the buffer to 0
        buffer.chars.iter_mut().for_each(|tab|
            tab.fill(Volatile::new(ScreenChar::default()))
        );

        Mutex::new(Writer {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::default(),
            buffer
        })
    };
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

        self.move_cursor_to_current_pos();

        Ok(())
    }
}

impl Writer {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\t' => self.move_column_position_by(4 - (self.column_position % 4)),
            b'\x08' => self.backspace(),

            byte => {
                let byte_to_write = match byte {
                    0x20..=0x7e => byte,
                    _ => 0xfe,
                };

                self.write_at_current_pos(byte_to_write);

                self.move_column_position_by(1);
            }
        }
    }

    fn write_at_current_pos(&mut self, byte: u8) {
        self.buffer.write(
            self.row_position,
            self.column_position,
            ScreenChar::new(byte, self.color_code),
        );
    }

    fn move_column_position_by(&mut self, n: usize) {
        let mut col_pos = self.column_position + n;
        while col_pos >= VGA_BUFFER_WIDTH {
            self.new_line();
            col_pos -= VGA_BUFFER_WIDTH;
        }
        self.column_position = col_pos;
    }

    fn scroll_down(&mut self) {
        for row in 1..VGA_BUFFER_HEIGHT {
            for col in 0..VGA_BUFFER_WIDTH {
                let char = self.buffer.read(row, col);
                self.buffer.write(row - 1, col, char);
            }
        }

        self.clear_row(VGA_BUFFER_HEIGHT - 1);
    }

    /// Clears a single row with blank characters
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::new(b' ', self.color_code);

        for col in 0..VGA_BUFFER_WIDTH {
            self.buffer.write(row, col, blank);
        }
    }

    pub fn move_cursor_pos(&mut self, col: usize, row: usize) {
        self.column_position = col.min(VGA_BUFFER_WIDTH - 1);
        self.row_position = row.min(VGA_BUFFER_HEIGHT - 1);
        self.move_cursor_to_current_pos();
    }

    fn move_cursor_to_current_pos(&mut self) {
        unsafe { vga::terminal_set_cursor(self.column_position, self.row_position) };
    }
}

// Key handlers
impl Writer {
    /// Moves to the next line, scrolling if necessary
    fn new_line(&mut self) {
        self.column_position = 0;
        if self.row_position < VGA_BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            self.scroll_down();
        }
    }

    pub fn backspace(&mut self) {
        if self.column_position == 0 {
            return;
        }

        self.column_position -= 1;

        self.write_at_current_pos(0);

        self.move_cursor_to_current_pos()
    }

    pub fn handle_right_arrow(&mut self) {
        if self.column_position >= VGA_BUFFER_WIDTH - 1 {
            return;
        }

        let current_char = self.buffer.read(self.row_position, self.column_position);
        if current_char.byte == 0 {
            return;
        }

        self.column_position += 1;
        self.move_cursor_to_current_pos()
    }

    pub fn handle_left_arrow(&mut self) {
        if self.column_position == 0 {
            return;
        }
        self.column_position -= 1;
        self.move_cursor_to_current_pos()
    }
}
