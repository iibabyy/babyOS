use crate::{
    idt::InterruptStackFrame,
    pic::{Irq, send_end_of_interrupt},
    print,
    shared::inb,
};

static mut LSHIFT_PRESSED: bool = false;
static mut RSHIFT_PRESSED: bool = false;
static mut CAPS_LOCK_ON: bool = false;
static mut IS_EXTENDED: bool = false;

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    let scancode = read_scancode();

    // if scancode < 0x80, a key was pressed down
    // if scancode > 0x80, a key was released
    match scancode {
        // left shift
        0x2A => set_left_shift_pressed(true),
        0xAA => set_left_shift_pressed(false), // 0x2A + 0x80 = 0xAA (Release)

        // right shift
        0x36 => set_right_shift_pressed(true),
        0xB6 => set_right_shift_pressed(false),

        // caps lock
        0x3A => toggle_caps_lock(),

        _ if next_scancode_extended() => {
            match scancode {
                0x4B => vga::writer::GLOBAL_WRITER.lock().handle_left_arrow(),
                0x4D => vga::writer::GLOBAL_WRITER.lock().handle_right_arrow(),

                0x53 => { /* DELETE (not backspace) */ }
                0x48 => { /* UP */ }
                0x50 => { /* DOWN */ }

                _ => {} // Ignore other extended keys (like Right Ctrl)
            }
        }

        // Printable keys
        0x00..=0x39 => print_scancode(scancode),

        _ => {}
    }

    // 0xE0 means that the next byte is an extended key (e.g. arrow keys, etc...)
    set_next_scancode_extended(scancode == 0xE0);

    send_end_of_interrupt(Irq::Keyboard);
}

pub fn read_scancode() -> u8 {
    const KEYBOARD_DATA_PORT: u16 = 0x60;
    unsafe { inb(KEYBOARD_DATA_PORT) }
}

fn print_scancode(scancode: u8) {
    let shift_pressed = is_left_shift_pressed() || is_right_shift_pressed();

    let mut c = if shift_pressed {
        SCANCODE_TO_SHIFTED_CHAR[scancode as usize]
    } else {
        SCANCODE_TO_CHAR[scancode as usize]
    };

    if is_caps_lock_on() && c.is_alphabetic() {
        if c.is_ascii_lowercase() {
            c = c.to_ascii_uppercase();
        } else {
            c = c.to_ascii_lowercase();
        }
    }

    if c != '\0' {
        print!("{c}")
    }
}

// Maps Scancodes 0x00 through 0x39 to ASCII characters
const SCANCODE_TO_CHAR: [char; 58] = [
    '\0', '\x1B', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=',
    '\x08', // 0x00 - 0x0E
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n', // 0x0F - 0x1C
    '\0', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`', // 0x1D - 0x29
    '\0', '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', '\0', '*', // 0x2A - 0x37
    '\0', ' ', // 0x38 - 0x39
];

// Maps Scancodes 0x00 through 0x39 when SHIFT is held down
const SCANCODE_TO_SHIFTED_CHAR: [char; 58] = [
    '\0', '\x1B', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '\x08', '\t', 'Q',
    'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', '\n', '\0', 'A', 'S', 'D', 'F', 'G',
    'H', 'J', 'K', 'L', ':', '"', '~', '\0', '|', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?',
    '\0', '*', '\0', ' ',
];

pub fn is_left_shift_pressed() -> bool {
    unsafe { LSHIFT_PRESSED }
}

pub fn set_left_shift_pressed(new: bool) {
    unsafe { LSHIFT_PRESSED = new }
}

pub fn is_right_shift_pressed() -> bool {
    unsafe { RSHIFT_PRESSED }
}

pub fn set_right_shift_pressed(new: bool) {
    unsafe { RSHIFT_PRESSED = new }
}

pub fn next_scancode_extended() -> bool {
    unsafe { IS_EXTENDED }
}

pub fn set_next_scancode_extended(new: bool) {
    unsafe { IS_EXTENDED = new }
}

pub fn is_caps_lock_on() -> bool {
    unsafe { CAPS_LOCK_ON }
}

pub fn toggle_caps_lock() {
    unsafe { CAPS_LOCK_ON = !CAPS_LOCK_ON }
}
