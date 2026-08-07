//! A minimal 32-bit x86 kernel library

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![warn(missing_docs)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(dead_code)]

mod gdt;
mod idt;
mod keyboard;
mod macros;
mod memory;
mod pic;
mod shared;
mod shell;
mod vga;

use core::arch::asm;
use core::panic::PanicInfo;

pub use self::gdt::dump::dump_kernel_stack;
pub use self::memory::{
	GRUB_MULTIBOOT_MAGIC,
	MultibootInfo,
	init_physical_memory,
	pmm_allocate_frame,
	pmm_deallocate_frame,
	enable_paging,
	init_virtual_memory,
};
pub use self::shell::shell_loop;
pub use self::vga::{
	_print,
	Color,
	GLOBAL_VGA_SCREEN,
};
use self::idt::interrupts::{disable_hardware_interrupts, enable_hardware_interrupts};

// TODO: add options to asm!() calls (for better compiler optimizations)

/// Initializes the kernel systems
pub fn init(magic_number: u32, multiboot_info_ptr: u32) {
	// validates magic_number
	assert_eq!(magic_number, GRUB_MULTIBOOT_MAGIC, "invalid magic number");

	gdt::init_gdt();
	pic::init_pics();

	// Safety: GDT is initialized
	unsafe { idt::init_idt() };

	// enables CPU hardware interrupts (e.g. keyboard keys)
	// Safety: IDT is initialized
	unsafe { enable_hardware_interrupts() };

	let multiboot_info_ptr = unsafe { &*(multiboot_info_ptr as *const MultibootInfo) };
	init_physical_memory(multiboot_info_ptr);

	// Safety: physical memory is initialized
	let directory_phys_addr = unsafe { init_virtual_memory() };

	unsafe {
		enable_paging(directory_phys_addr);
	}
}

/// Prints the panic info and enters an infinite loop
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
	println!("{info}");

	loop {
		disable_hardware_interrupts();
		unsafe {
			asm!("hlt");
		}
	}
}
