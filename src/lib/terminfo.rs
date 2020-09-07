use libc::{
	c_ushort, 
	c_int, 
	ioctl, 
	winsize, 
	STDERR_FILENO, 
	STDIN_FILENO, 
	STDOUT_FILENO, 
	TIOCGWINSZ
};

pub struct TermDimensions {
	pub cols: usize,
	pub rows: usize
}

pub fn dim() -> Result<TermDimensions, &'static str> {
	let window = get_term_size_any();
	match window {
		Some(win) => Ok(TermDimensions { 
			cols: win.ws_col as usize, 
			rows: win.ws_row as usize 
		}),
		None => Err("Unable to check window dimensions.")
	}
}

fn get_term_size_any() -> Option<winsize> {
	if let Some(window) = get_term_size(STDOUT_FILENO) {
		return Some(window);
	}
	if let Some(window) = get_term_size(STDIN_FILENO) {
		return Some(window);
	}
	if let Some(window) = get_term_size(STDERR_FILENO) {
		return Some(window);
	}
	None
}

fn get_term_size(output: c_int) -> Option<winsize> {
	let mut window = initialize_winsize();
	let result = unsafe { ioctl(output, TIOCGWINSZ, &mut window) };
	match result {
		-1 => None,
		_ => Some(window),
	}
}

fn initialize_winsize() -> winsize {
	winsize {
		ws_row: 0 as c_ushort,
		ws_col: 0 as c_ushort,
		ws_xpixel: 0 as c_ushort,
		ws_ypixel: 0 as c_ushort,
	}
}
