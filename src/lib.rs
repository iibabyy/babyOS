#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
 
use core::panic::PanicInfo;

pub mod vga;
pub mod macros;
mod interrupts;
mod pic;

pub fn init() {
    pic::init_pics();
    interrupts::idt::init_idt();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    loop {}
}
