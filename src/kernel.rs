#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

// #![warn(missing_docs)]

use baby_lib::panic;

mod io;

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    #[cfg(test)]
    tests::run_tests();

    println!("1");
    println!("2");
    println!("3");
    println!("4");
    println!("5");
    println!("6");
    println!("7");
    println!("8");
    println!("9");
    println!("10");
    println!("11");
    println!("12");
    println!("13");
    println!("14");
    println!("15");
    println!("16");
    println!("17");
    println!("18");
    println!("19");
    println!("20");
    println!("21");
    println!("22");
    println!("23");
    println!("24");
    println!("25");
    println!("26");
    println!("27");
    println!("28");
    println!("29");
    println!("30");
    println!("31");
    println!("32");
    println!("33");
    println!("34");
    println!("35");
    println!("36");
    println!("37");
    println!("38");
    println!("39");
    println!("40");
    println!("41");
    println!("42");

    loop {}
}
