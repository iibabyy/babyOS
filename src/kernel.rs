//! A minimal 32-bit x86 kernel

#![no_main]
#![no_std]

use core::hint;

use kernel::{
	dump_kernel_stack,
	pmm_allocate_frame,
	pmm_deallocate_frame,
	println,
};

/// First rust function called by the link.ld file
#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint(magic_number: u32, multiboot_info_ptr: u32) -> ! {
	kernel::init(magic_number, multiboot_info_ptr);

	let test = unsafe { *(0x80000000 as *const u32) };

	hint::black_box(test);

	kernel::shell_loop();
}

#[inline(never)]
pub fn test_my_stack() {
	let a: u32 = 0xdeadbeef;
	let b: u32 = 0xcafebabe;

	crate::println!("Variables live at: {:p} and {:p}", &a, &b);

	dump_kernel_stack();

	// so the compiler don't drop them too early
	core::hint::black_box(a);
	core::hint::black_box(b);
}
