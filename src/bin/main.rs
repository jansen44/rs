use std::{process, fs, env};
use std::path::Path;

enum ExitCodes {
    General = 1,
    CannotExecute = 126
}

fn main() {
    let mut args = env::args();
    if args.len() > 2 {
        println!("Something went wrong! Multiple directories are not supported yet");
        process::exit(ExitCodes::CannotExecute as i32);
    }
    
    let str_path = if args.len() == 1 { String::from(".") } 
    else { 
        match args.nth(1) {
            Some(path) => path,
            None => {
                println!("Something went wrong! Could not get the first arg correctly!");
                process::exit(ExitCodes::General as i32);
            }
        }
    };

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

    println!("{:?}", metadata);
}

fn check_symlink(path: &Path) -> Result<(), ()> {
    match fs::read_link(path) {
        Err(_) => Ok(()),
        _ => Err(())
    }
}

