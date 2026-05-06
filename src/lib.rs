#![no_std]

use core::panic::PanicInfo;

pub mod io;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    loop {}
}
