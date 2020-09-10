pub const ARG_NAME_LIST: &str = "List";
pub const ARG_NAME_ALL: &str = "All";
pub const ARG_NAME_HELP: &str = "Help";
pub const ARG_NAME_DIR_FIRST: &str = "Don't print directories first";

pub const COLS_MULTIPLIER_FOR_GRID: f32 = 1.5;

pub enum ExitCodes {
	Success = 0,
	CannotExecute = 126
}
