use crate::util;
use crate::cowmand;
use std::{process, path, fs, io, ffi::OsString};

#[derive(Debug)]
pub struct App<'app> {
	cow: cowmand::Cowmand<'app>,
	paths: Vec<path::PathBuf>,
	show_help: bool,
	show_all: bool,
	show_list: bool
}

impl<'app> App<'app> {
	pub fn new(cow: cowmand::Cowmand<'app>) -> Self {
		let _cow = cow.clone();
		let mut show_help = false;
		let mut show_all = false;
		let mut show_list = false;
		let mut paths = Vec::<path::PathBuf>::new();
		if let Some(flags) = cow.active_flags {
			for flag in flags.iter() {
				match flag.as_str() {
					util::ARG_NAME_HELP => show_help = true,
					util::ARG_NAME_ALL  => show_all  = true,
					util::ARG_NAME_LIST => show_list = true,
					_ => ()
				};
			}
		}
		if let Some(args) = cow.active_args {
			for arg in args.iter() { paths.push(App::get_path(arg)); }
		}
		App { cow: _cow, show_help, show_all, show_list, paths }
	}

	pub fn run(&self) {
		if self.show_help {
			print!("{}", self.cow);
			App::success();
		}

		match self.paths.len() {
			0 => match self.path_handler(&App::get_path(".")) {
				Err(e) => println!("rs: something went wrong: {}", e),
				_ => App::success()
			},
			1 => match self.path_handler(&self.paths[0]) {
				Err(e) => println!("rs: something went wrong: {}", e),
				_ => App::success()
			},
			_ => ()
		};
	}

	pub fn path_handler(&self, path: &path::Path) -> io::Result<()> {
		// ToDo: Better error handling for dangling unwraps
		let metadata = path.metadata()?;
		if metadata.is_file() {
			println!(
				"{}", 
				path.file_name().unwrap()
					.to_str().unwrap()
			);
			return Ok(());
		}
		self.dir_routine(path)?;
		Ok(())
	}

	pub fn dir_routine(&self, path: &path::Path) -> io::Result<()> {
		// ToDo: Better error handling for dangling unwraps
		for entry in fs::read_dir(path)? {
			let unwrapped_entry = entry?;
			// ToDo: Better way to check whether the file is hidden or not
			if App::first_os_string_char(unwrapped_entry.file_name()) != '.' 
			|| (App::first_os_string_char(unwrapped_entry.file_name()) == '.' && self.show_all) {
				let metadata = unwrapped_entry.metadata()?;
				print!(
					"{}{} ", 
					unwrapped_entry.file_name().to_str().unwrap(),
					if metadata.is_dir() { "/" } else { "" }
				);
			}
		}
		println!();
		Ok(())
	}

	pub fn first_os_string_char(file_name: OsString) -> char {
		match file_name.to_str().unwrap().chars().next() {
			Some(c) => c,
			None => ' '
		}
	}

	fn get_path(possible_path: &'app str) -> path::PathBuf {
		let path_buf = path::PathBuf::from(&possible_path);
		if let Ok(_) = fs::read_link(&path_buf) {
			println!("rs: something went wrong: \
				Symlinks are not supported yet!"
			);
			App::cant_execute();
		}
		path_buf
	}

	fn success() {
		process::exit(util::ExitCodes::Success as i32);
	}
	
	fn cant_execute() {
		process::exit(util::ExitCodes::CannotExecute as i32);
	}
}

