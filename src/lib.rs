#![no_std]

use core::panic::PanicInfo;

pub mod io;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    loop {}
}
