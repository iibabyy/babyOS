#![no_main]
#![no_std]

use baby_lib::panic;

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    loop {}
}
