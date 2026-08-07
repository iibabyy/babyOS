//! A minimal 32-bit x86 kernel

#![no_main]
#![no_std]

/// First rust function called by the link.ld file
#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint(magic_number: u32, multiboot_info_ptr: u32) -> ! {
	kernel::init(magic_number, multiboot_info_ptr);

	kernel::shell_loop();
}
