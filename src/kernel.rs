#![no_main]
#![no_std]
// #![warn(missing_docs)]

mod io;

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    println!("Hello Word{}", " !");

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}