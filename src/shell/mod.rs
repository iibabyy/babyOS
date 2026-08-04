use core::{arch::asm, str};

use crate::{GLOBAL_VGA_SCREEN, dump_kernel_stack, idt, println, shared::outb, vga::VGA_BUFFER_WIDTH};

static mut COMMAND_BUFFER: [u8; VGA_BUFFER_WIDTH] = [0; VGA_BUFFER_WIDTH];
static mut COMMAND_LENGTH: usize = 0;
static mut COMMAND_READY: bool = false;

/// Runs an interactive command interpreter loop
/// 
/// available commands are stack, halt, reboot, and clear
pub fn shell_loop() -> ! {
    loop {
        unsafe {
            asm!("hlt"); // freeze until next interrupt

            if COMMAND_READY == false {
                continue;
            }

            match str::from_utf8_unchecked(&COMMAND_BUFFER[0..COMMAND_LENGTH]) {
                "stack" => dump_kernel_stack(),

                "halt" => {
                    println!("System halted");
                    idt::disable_hardware_interrupts();
                    asm!("hlt"); // freeze CPU
                }

                "reboot" => {
                    println!("Rebooting...");
                    outb(0x64, 0xFE); // magic trick :)
                }

				"clear" => GLOBAL_VGA_SCREEN.lock().clear(),

                str if COMMAND_LENGTH > 0 => println!("Unknown command: {str}"),
                _ => {}
            }

            reset_cmd_buffer();
        }
    }
}

/// Adds a character to [COMMAND_BUFFER]
/// 
/// handles backspaces and sets [COMMAND_READY] to `true` on a newline
pub fn add_char_to_command_buffer(c: u8) {

    // backspace
    unsafe {
        match c {
            b'\x08' => {
                if COMMAND_LENGTH > 0 {
                    COMMAND_BUFFER[COMMAND_LENGTH] = 0;
                    COMMAND_LENGTH -= 1;
                }
            }

            b'\n' => COMMAND_READY = true,

            _ => {
                if COMMAND_LENGTH >= VGA_BUFFER_WIDTH {
                    reset_cmd_buffer();
                }
                COMMAND_BUFFER[COMMAND_LENGTH] = c;
                COMMAND_LENGTH += 1;
            }
        }
    }
}

fn reset_cmd_buffer() {
    unsafe {
        COMMAND_LENGTH = 0;
        COMMAND_READY = false;
    }
}
