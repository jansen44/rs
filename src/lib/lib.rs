use std::{env, process, path, fs, fmt};

pub enum ExitCodes {
    Success = 0,
    CannotExecute = 126
}

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
        // Temp: Handle args quantity
        if args.len() > 2 {
            println!("Something went wrong! Multiple directories are not supported yet");
            process::exit(ExitCodes::CannotExecute as i32);
        }

        let path = get_path(&mut args);
        let metadata = path.metadata().unwrap_or_else(|err| {
            println!("Something went wrong for '{}'! {}", path.display(), err);
            process::exit(ExitCodes::CannotExecute as i32);
        });

        RsApp { path, metadata }
    }

    pub fn run(&self) {
        if self.metadata.is_file() {
            println!("{}\n", self.path.display());
            self.success();
        }

        for entry in fs::read_dir(&self.path).expect("Can't get file list") {
            print!("{} ", entry.unwrap().path().display());
        }
        println!("");
    }

    fn success(&self) {
        process::exit(ExitCodes::Success as i32);
    }
}

fn get_path(args: &mut env::Args) -> path::PathBuf {
    let str_path = args.nth(1).unwrap_or(String::from("."));
    let path = path::PathBuf::from(&str_path);
    if let Ok(_) = fs::read_link(&path) {
        println!("Something went wrong! SymLinks are not supported yet");
        process::exit(ExitCodes::CannotExecute as i32);
    }
    path
}

