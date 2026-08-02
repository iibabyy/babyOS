#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![expect(unused)]

use core::panic::PanicInfo;

mod gdt;
mod idt;
mod keyboard;
mod macros;
mod pic;
mod shared;

mod vga;
pub use vga::{_print, writer::GLOBAL_WRITER, color_code::Color};

pub fn init() {
    gdt::init_gdt();
    pic::init_pics();
    idt::init_idt();
    idt::enable();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    loop {}
}
