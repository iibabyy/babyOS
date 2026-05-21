#![no_main]
#![no_std]

// #![warn(missing_docs)]

use core::arch::asm;

use baby_lib::{interrupts, panic, print, println};

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {

	#[cfg(test)]
	{
		baby_lib::run_lib_tests();
		loop {}
	}

	interrupts::idt::init_idt();

	unsafe { asm!("int3"); }

    println!("1");
    println!("2");
    println!("3");

    loop {}
}
