use crate::{io::vga::WRITER, println};

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
		let writer = WRITER.lock();
        let screen_char = writer.buffer.chars[writer.row_position][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}