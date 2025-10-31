#![no_main]
#![no_std]
// #![warn(missing_docs)]

mod io;

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    use core::fmt::Write;

    vga::WRITER.lock().write_str("Hello again\n").unwrap();
    write!(vga::WRITER.lock(), "some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}

use core::panic::PanicInfo;

use crate::io::vga;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}