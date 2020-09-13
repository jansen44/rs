pub enum Colors {
	Black = 0,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
	LightBlack,
	LightRed,
	LightGreen,
	LightYellow,
	LightBlue,
	LightMagenta,
	LightCyan,
	LightWhite,
}

impl Colors {
	pub fn fg(self) -> String {
		format!("\x1B[38;5;{}m", self as u32)
	}
	pub fn bg(self) -> String {
		format!("\x1B[48;5;{}m", self as u32)
	}
}
