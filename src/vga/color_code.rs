/// VGA colors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[expect(missing_docs)]
pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}

/// VGA color code containing foreground and background colors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
	#[must_use]
	pub const fn new(foreground: Color, background: Color) -> ColorCode {
		ColorCode((background as u8) << 4 | (foreground as u8))
	}

	#[must_use]
	pub const fn white_on_black() -> ColorCode {
		ColorCode::new(Color::White, Color::Black)
	}

	pub const fn set_background_color(&mut self, background: Color) {
		self.0 = (background as u8) << 4 | (self.0 & 0x0f);
	}

	pub const fn set_foreground_color(&mut self, foreground: Color) {
		self.0 = (self.0 & 0xf0) | (foreground as u8);
	}
}
