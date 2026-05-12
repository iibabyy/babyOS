#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        #[allow(clippy::used_underscore_items)]
        $crate::io::vga::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
