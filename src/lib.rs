#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod idt;
mod keyboard;
mod macros;
mod pic;
mod shared;

mod memory;
pub use memory::{pmm_allocate_frame, pmm_deallocate_frame};

mod shell;
pub use shell::shell_loop;

mod gdt;
pub use gdt::dump::dump_kernel_stack;

mod vga;
pub use vga::{_print, Color, GLOBAL_WRITER};

use crate::memory::{GRUB_MULTIBOOT_MAGIC, MultibootInfo, init_physical_memory};


pub fn init(magic_number: u32, multiboot_info_ptr: u32) {
	// validates magic_number
	assert_eq!(magic_number, GRUB_MULTIBOOT_MAGIC, "invalid magic number");

    gdt::init_gdt();
    pic::init_pics();

	unsafe {
		// SAFETY: GDT is initialized
        idt::init_idt();

		// enables CPU hardware interrupts (e.g. keyboard keys)
		// SAFETY: IDT is initialized
        idt::enable_hardware_interrupts();
    }

	let multiboot_info_ptr = unsafe { &*(multiboot_info_ptr as *const MultibootInfo) };
	init_physical_memory(multiboot_info_ptr);
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    loop {}
}
