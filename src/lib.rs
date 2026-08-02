#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod idt;
mod keyboard;
mod macros;
mod pic;
mod shared;

mod shell;
pub use shell::shell_loop;

mod gdt;
pub use gdt::dump::dump_kernel_stack;

mod vga;
pub use vga::{_print, Color, GLOBAL_WRITER};

pub fn init() {
    gdt::init_gdt();
    pic::init_pics();
    unsafe {
        idt::init_idt();
        idt::enable_hardware_interrupts();
    }
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    loop {}
}
