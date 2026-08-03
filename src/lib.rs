#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod gdt;
mod idt;
mod keyboard;
mod macros;
mod pic;
mod shared;

mod vga;
pub use vga::{_print, Color, GLOBAL_WRITER};

pub fn init() {
    gdt::init_gdt();
    pic::init_pics();
    
	unsafe {
		// SAFETY: GDT is initialized
        idt::init_idt();

		// enables CPU hardware interrupts (e.g. keyboard keys)
		// SAFETY: IDT is initialized
        idt::enable();
    }    
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    loop {}
}
