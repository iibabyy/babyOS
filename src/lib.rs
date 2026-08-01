#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![expect(unused)]

use core::panic::PanicInfo;

mod gdt;
mod idt;
pub mod macros;
mod pic;
mod shared;
pub mod vga;

pub fn init() {
    gdt::init_gdt();
    pic::init_pics();
    idt::init_idt();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    loop {}
}
