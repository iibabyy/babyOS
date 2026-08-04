/// Prints to the standard output.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        #[allow(clippy::used_underscore_items)]
        $crate::_print(format_args!($($arg)*))
    }};
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
