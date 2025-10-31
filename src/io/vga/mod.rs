mod buffer;
mod color;
mod writer;

use writer::Writer;
use color::{Color, ColorCode};
use buffer::Buffer;

use lazy_static::lazy_static;

lazy_static! {
	pub static ref WRITER: spin::Mutex<Writer> = spin::Mutex::new(
		Writer {
			column_position: 0,
			color_code: ColorCode::new(Color::Yellow, Color::Black),
			buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
		}
	);
}