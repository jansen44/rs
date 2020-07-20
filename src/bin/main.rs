use std::{process, fs, env};
use std::path::Path;

enum ExitCodes {
    Success = 0,
    CannotExecute = 126
}

fn main() {
    let mut args = env::args();
    if args.len() > 2 {
        println!("Something went wrong! Multiple directories are not supported yet");
        process::exit(ExitCodes::CannotExecute as i32);
    }
    
    let str_path = if args.len() == 1 { String::from(".") } 
    else { args.nth(1).expect("Could not get the first arg correctly") };

    let path = Path::new(&str_path);
    if let Err(_) = check_symlink(path) {
        println!("Something went wrong! Symlinks are not supported yet");
        process::exit(ExitCodes::CannotExecute as i32);
    }

    let metadata = match path.metadata() {
        Ok(meta) => meta,
        Err(e) => {
            println!("Something went wrong for '{}'! {}", path.display(), e);
            process::exit(ExitCodes::CannotExecute as i32);
        }
    };

    if metadata.is_file() {
        println!("{}", path.display());
        process::exit(ExitCodes::Success as i32);
    }
    
    for entry in fs::read_dir(path).expect("Can't get file list") {
        print!("{} ", entry.unwrap().path().display());
    }
    println!("");
}

fn check_symlink(path: &Path) -> Result<(), ()> {
    match fs::read_link(path) {
        Err(_) => Ok(()),
        _ => Err(())
    }
}

