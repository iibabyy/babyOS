#![no_main]
#![no_std]

// #![warn(missing_docs)]

use core::arch::asm;

use baby_lib::{Color, GLOBAL_WRITER, dump_kernel_stack, println};

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    baby_lib::init();

    // test_my_stack();
    // println!("42");

    // unsafe {
    //     asm!("int3");
    // }

    // println!("1");
    // println!("2");
    // println!("3");
    // println!("");

    // GLOBAL_WRITER
    //     .lock()
    //     .color_code
    //     .set_foreground_color(Color::Cyan);
    // GLOBAL_WRITER
    //     .lock()
    //     .color_code
    //     .set_background_color(Color::White);
    // println!("1");
    // println!("2");
    // println!("3");
    // println!("");

    // GLOBAL_WRITER
    //     .lock()
    //     .color_code
    //     .set_foreground_color(Color::White);
    // GLOBAL_WRITER
    //     .lock()
    //     .color_code
    //     .set_background_color(Color::Black);
    // println!("1");
    // println!("2");
    // println!("3");
    // println!("");

    baby_lib::shell_loop();
}

#[inline(never)]
pub fn test_my_stack() {
    let a: u32 = 0xDEADBEEF;
    let b: u32 = 0xCAFEBABE;

    crate::println!("Variables live at: {:p} and {:p}", &a, &b);

    dump_kernel_stack();

    // so the compiler don't drop them too early
    core::hint::black_box(a);
    core::hint::black_box(b);
}
