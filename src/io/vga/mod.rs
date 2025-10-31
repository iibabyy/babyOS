mod buffer;
mod color;
mod writer;

use writer::Writer;
use color::{Color, ColorCode};
use buffer::Buffer;

use lazy_static::lazy_static;
use spin;

lazy_static! {
	pub static ref WRITER: spin::Mutex<Writer> = spin::Mutex::new(
		Writer {
			column_position: 0,
			color_code: ColorCode::new(Color::White, Color::Black),
			buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
		}
	);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
