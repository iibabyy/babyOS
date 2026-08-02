#![expect(unused_imports)]

mod buffer;
pub use buffer::{Buffer, ScreenChar, VGA_BUFFER_ADDRESS, VGA_BUFFER_HEIGHT, VGA_BUFFER_WIDTH};

mod color_code;
pub use color_code::{Color, ColorCode};

mod cursor;
pub use cursor::terminal_set_cursor;

mod screens;
pub use screens::{handle_shortcut_switch_screen, switch_screen};

mod writer;
pub use writer::{GLOBAL_WRITER, Writer};

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    writer::GLOBAL_WRITER.lock().write_fmt(args).unwrap();
}
