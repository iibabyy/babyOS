use core::arch::asm;

// dump the stack of the caller function
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

    crate::println!("=== KERNEL STACK DUMP ===");
    while current_ptr <= caller_ebp {
        let value = unsafe { *(current_ptr as *const u32) };

		crate::println!("{:#010X}: {:#010X}", current_ptr, value);
        
        current_ptr += 4;

		unsafe {asm!("hlt")};
    }
    crate::println!("=========================");
}
