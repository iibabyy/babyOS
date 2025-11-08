use baby_lib::io::vga::{
    buffer::*,
    writer::{KernelWriter, KernelWriterInfos},
    *,
};
use volatile::Volatile;

use crate::*;

pub struct TestWriter {
    pub infos: KernelWriterInfos,
    pub buffer: Buffer,
}

impl core::fmt::Write for TestWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl KernelWriter for TestWriter {
    fn infos(&mut self) -> &mut KernelWriterInfos {
        &mut self.infos
    }

    fn read(&self, row: usize, col: usize) -> ScreenChar {
        self.buffer.chars[row][col].read()
    }

    fn write(&mut self, row: usize, col: usize, byte: ScreenChar) {
        self.buffer.chars[row][col].write(byte);
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    /// Global writer instance for safe console output
    /// Provides thread-safe access to the VGA text buffer
    pub static ref MOCK_WRITER: Mutex<TestWriter> = Mutex::new(
        TestWriter {
            infos: KernelWriterInfos {
                column_position: 0,
                row_position: 0,
                color_code: ColorCode::new(Color::White, Color::Black),
            },
            buffer: Buffer {
                chars: {
                    core::array::from_fn(|_|        // --> [[...]; BUFFER_HEIGHT]
                        core::array::from_fn(|_|    // --> [Volatile<ScreenChar>; BUFFER_WIDTH]
                            Volatile::new(ScreenChar {
                                ascii_character: b' ',
                                color_code: ColorCode::new(Color::White, Color::Black),
                            })
                        )
                    )
                }
            }
        }
    );
}
