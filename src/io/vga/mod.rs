pub mod buffer;
pub mod color_code;
pub mod cursor;
pub mod writer;

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    writer::GLOBAL_WRITER.lock().write_fmt(args).unwrap();
}
