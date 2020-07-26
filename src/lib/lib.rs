pub mod util;
pub mod cowmand;

use std::{env, process, path, fs, fmt};

pub struct RsApp {
    path: path::PathBuf,
    metadata: fs::Metadata
}

impl fmt::Display for RsApp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "path: \"{}\", metadata: {:?} )", self.path.display(), self.metadata)
    }
}

impl RsApp {
    pub fn new() -> RsApp {
        let mut args = env::args();
        // ToDo: Handle args quantity
        if args.len() > 2 {
            println!("Something went wrong! Multiple directories are not supported yet");
            process::exit(util::ExitCodes::CannotExecute as i32);
        }

        let path = get_path(&mut args);
        let metadata = path.metadata().unwrap_or_else(|err| {
            println!("Something went wrong for '{}'! {}", path.display(), err);
            process::exit(util::ExitCodes::CannotExecute as i32);
        });

        RsApp { 
            path: path, 
            metadata: metadata
        }
    }

    pub fn run(&self) {
        if self.metadata.is_file() {
            let file_name = self.path.file_stem().unwrap();
            println!("{}\n", file_name.to_str().unwrap());
            self.success();
        }

        self.dir_routine()
    }


    pub fn dir_routine(&self) {
        // ToDo: Better error handling
        for entry in fs::read_dir(&self.path).expect("Can't get file list") {
            let file_name = entry.unwrap().file_name();
            print!("{} ", file_name.to_str().unwrap());
        }
        println!("");
    }

    fn success(&self) {
        process::exit(util::ExitCodes::Success as i32);
    }
}

fn get_path(args: &mut env::Args) -> path::PathBuf {
    let str_path = args.nth(1).unwrap_or(String::from("."));
    let path = path::PathBuf::from(&str_path);
    if let Ok(_) = fs::read_link(&path) {
        println!("Something went wrong! SymLinks are not supported yet");
        process::exit(util::ExitCodes::CannotExecute as i32);
    }
    path
}

