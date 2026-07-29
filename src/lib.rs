#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
 
use core::panic::PanicInfo;

pub mod io;
pub mod macros;
pub mod interrupts;

pub fn init() {
    interrupts::idt::init_idt();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    loop {}
}
