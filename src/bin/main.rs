use rs::app::App;
use rs::cowmand::{Arg, Cowmand};
use rs::util::{
	ARG_NAME_ALL, 
	ARG_NAME_HELP, 
	ARG_NAME_LIST,
	ARG_NAME_DIR_FIRST
};

fn main() {
	App::new(
		Cowmand::new("rs")
			.description("A (almost) zero-dependency 'ls' alternative.")
			.arg(
				Arg::new(ARG_NAME_LIST)
					.short_command('l')
					.long_command("list")
					.help("show files/directories in list mode."),
			)
			.arg(
				Arg::new(ARG_NAME_ALL)
					.short_command('a')
					.long_command("all")
					.help("show all files/directories."),
			)
			.arg(
				Arg::new(ARG_NAME_HELP)
					.short_command('h')
					.long_command("help")
					.help("show this help message."),
			)
			.arg(
				Arg::new(ARG_NAME_DIR_FIRST)
					.short_command('d')
					.long_command("no-dir")
			)
			.get_args(),
	)
	.run();
}
