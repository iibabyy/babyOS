#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

// #![warn(missing_docs)]

#[cfg(test)]
mod tests;
mod io;

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    #[cfg(test)]
    tests::run_tests();

    println!("Hello Word{}", " !");
    println!("Hello Word{}", " !");
    println!("Hello Word{}", " !");
    println!("Hello Word{}", " !");
    println!("Hello Word{}", " !");
    println!("Hello Word{}", " !");
    println!("Hello Word{}", " !");
    println!("Hello Word{}", " !");
    println!("Hello Word{}", " !");
    println!("Hello Word{}", " !");
    println!("Hello Word{}", " !");
    println!("Hello Word{}", " !");

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    loop {}
}
