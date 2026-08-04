//! A minimal 32-bit x86 kernel

#![no_main]
#![no_std]

use kernel::{
	dump_kernel_stack,
	pmm_allocate_frame,
	pmm_deallocate_frame,
	println,
};

/// First rust function called by the link.ld file
#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint(magic_number: u32, multiboot_info_ptr: u32) -> ! {
	baby_lib::init(magic_number, multiboot_info_ptr);

	if let Some(frame_addr) = pmm_allocate_frame() {
		let address = unsafe { &mut *(frame_addr as *const u8 as *mut u8) };

		*address = 12;
		println!("Successfuly allocated memory {address:p} and writed {address} in it");
		pmm_deallocate_frame(frame_addr);
		println!("Successfuly deallocated memory");
	}

	baby_lib::shell_loop();
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
