#![no_main]
#![no_std]
// #![warn(missing_docs)]

mod io;

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    vga::writer::print_something();

    loop {}
}

use core::panic::PanicInfo;

use crate::io::vga;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}