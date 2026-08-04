#![no_main]
#![no_std]

use kernel::panic;

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    loop {}
}
