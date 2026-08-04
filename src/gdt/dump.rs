use core::arch::asm;

use crate::println;

/// Dumps the stack frames of the caller function
pub fn dump_kernel_stack() {
	let mut ebp: u32;

	unsafe {
		asm!(
			"mov {0}, ebp",
			out(reg) ebp
		);
	}

	// read the memory at ebp to find the caller's ebp
	let caller_ebp = unsafe { *(ebp as *const u32) };
	let mut current_ptr = ebp + 4;

	let mut lines = 0;
	let mut hidden = 0;

	crate::println!("=== KERNEL STACK DUMP ===");
	while current_ptr <= caller_ebp {
		let value = unsafe { *(current_ptr as *const u32) };

		if lines < 15 {
			crate::println!("{:#010X}: {:#010X}", current_ptr, value);
		} else {
			hidden += 1;
		}

		current_ptr += 4;
		lines += 1;
	}

	if hidden > 0 {
		println!("+{hidden}...");
	}

	crate::println!("=========================");
}
