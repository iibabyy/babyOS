#![no_main]
#![no_std]

// #![warn(missing_docs)]

use core::arch::asm;

use baby_lib::println;

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {

    baby_lib::init();

	unsafe { asm!("int3"); }

    println!("1");
    println!("2");
    println!("3");
    println!("");

    println!("1");
    println!("2");
    println!("3");
    println!("");

    println!("1");
    println!("2");
    println!("3");
    println!("");

    loop {}
}
