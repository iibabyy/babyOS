#![no_main]
#![no_std]

// #![warn(missing_docs)]

use core::arch::asm;

use baby_lib::{Color, GLOBAL_WRITER, println};

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    baby_lib::init();

    unsafe {
        asm!("int3");
    }

    println!("1");
    println!("2");
    println!("3");
    println!("");

    GLOBAL_WRITER.lock().color_code.set_foreground_color(Color::Cyan);
    GLOBAL_WRITER.lock().color_code.set_background_color(Color::White);
    println!("1");
    println!("2");
    println!("3");
    println!("");

    GLOBAL_WRITER.lock().color_code.set_foreground_color(Color::White);
    GLOBAL_WRITER.lock().color_code.set_background_color(Color::Black);
    println!("1");
    println!("2");
    println!("3");
    println!("");

    loop {
        // puts the CPU to sleep until the next interrupt fires
        unsafe {
            asm!("hlt");
        }
    }
}
