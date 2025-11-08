#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![cfg(test)]

use baby_lib::{print, println};

mod io;
mod utils;

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
	self::test_main();
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    use baby_lib::println;

    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}