//! A minimal 32-bit x86 kernel

#![no_main]
#![no_std]

use kernel::{
	alloc::boxed::Box, println
};

/// First rust function called by the link.ld file
#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint(magic_number: u32, multiboot_info_ptr: u32) -> ! {
	kernel::init(magic_number, multiboot_info_ptr);

	let x = Box::new(41);

	println!("just created {x} !");

	kernel::shell_loop();
}
