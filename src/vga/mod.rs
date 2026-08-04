#![expect(unused_imports)]

mod buffer;
use core::fmt::Write;

pub use buffer::{
	Buffer,
	ScreenChar,
	VGA_BUFFER_ADDRESS,
	VGA_BUFFER_HEIGHT,
	VGA_BUFFER_WIDTH,
};

mod color_code;
pub use color_code::{
	Color,
	ColorCode,
};

mod cursor;
pub use cursor::terminal_set_cursor;

mod screens;
use lazy_static::lazy_static;
pub use screens::{
	handle_shortcut_switch_screen,
	switch_screen,
};
use spin::Mutex;
use volatile::Volatile;

pub use crate::vga::buffer::{
	Buffer,
	ScreenChar,
	VGA_BUFFER_ADDRESS,
	VGA_BUFFER_HEIGHT,
	VGA_BUFFER_WIDTH,
};
pub use crate::vga::color_code::{
	Color,
	ColorCode,
};
pub use crate::vga::cursor::terminal_set_cursor;
pub use crate::vga::screens::{
	handle_shortcut_switch_screen,
	switch_screen,
};

lazy_static! {
	/// Global [VgaScreen] protected by a [Mutex] to write to the VGA screen
	pub static ref GLOBAL_VGA_SCREEN: Mutex<VgaScreen> = {

		let mut writer = VgaScreen {
			column_position: 0,
			row_position: 0,
			color_code: ColorCode::white_on_black(),
			buffer: unsafe { &mut *(VGA_BUFFER_ADDRESS as *mut Buffer) },
		};

		writer.clear();

		Mutex::new(writer)
	};
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
	use core::fmt::Write;
	GLOBAL_VGA_SCREEN.lock().write_fmt(args).unwrap();
}

/// Holds a reference to the VGA buffer
/// and informations about the current writing state
pub struct VgaScreen {
	pub column_position: usize,
	pub row_position: usize,

	/// foreground and background color code
	pub color_code: ColorCode,

	/// VGA screen buffer
	pub buffer: &'static mut Buffer,
}

impl Write for VgaScreen {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		for byte in s.bytes() {
			self.write_byte(byte);
		}

		self.move_cursor_to_current_pos();

		Ok(())
	}
}

impl VgaScreen {
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
		// SAFETY: args are not (should not be 👀) out of bounds
		unsafe { vga::terminal_set_cursor(self.column_position, self.row_position) };
	}

	/// Fills the VGA screen buffer with null bytes
	pub fn clear(&mut self) {
		for row in self.buffer.chars.iter_mut() {
			row.fill(Volatile::new(ScreenChar::new(0x0, self.color_code)));
		}

		self.column_position = 0;
		self.row_position = 0;
		self.move_cursor_to_current_pos();
	}
}

// Key handlers
impl VgaScreen {
	/// Moves to the next line, scrolling if necessary
	fn new_line(&mut self) {
		self.column_position = 0;
		if self.row_position < VGA_BUFFER_HEIGHT - 1 {
			self.row_position += 1;
		} else {
			self.scroll_down();
		}
	}

	/// Deletes the character behind the cursor
	pub fn backspace(&mut self) {
		if self.column_position == 0 {
			return;
		}

		self.column_position -= 1;

		self.write_at_current_pos(0);

		self.move_cursor_to_current_pos()
	}

	/// Moves the cursor one character to the right
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

	/// Moves the cursor one character to the left
	pub fn handle_left_arrow(&mut self) {
		if self.column_position == 0 {
			return;
		}
		self.column_position -= 1;
		self.move_cursor_to_current_pos()
	}
}
