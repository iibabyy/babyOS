//! VGA Text Buffer Module
//!
//! Handles low-level text output to the VGA buffer at 0xb8000
//! Provides thread-safe console output through a global writer instance

use lazy_static::lazy_static;
use spin::Mutex;

pub mod buffer;
pub mod writer;

// ================================
// Constants
// ================================

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

// ================================
// Global Writer Instance
// ================================

use crate::io::vga::{
    buffer::{Color, ColorCode},
    writer::{KernelWriterInfos, Writer},
};

lazy_static! {
    /// Global writer instance for safe console output
    /// Provides thread-safe access to the VGA text buffer
    pub static ref WRITER: Mutex<Writer> = Mutex::new(
        Writer {
            infos: KernelWriterInfos {
                column_position: 0,
                row_position: 0,
                color_code: ColorCode::new(Color::White, Color::Black),
            },
            buffer: unsafe { &mut *(0xb8000 as *mut buffer::Buffer) }
        }
    );
}

// ================================
// Public Interface Functions
// ================================

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    crate::io::vga::WRITER.lock().write_fmt(args).unwrap();
}
