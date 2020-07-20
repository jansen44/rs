use std::{process, fs};
use std::path::Path;

const EXIT_CANNOT_EXECUTE: i32 = 126;

fn main() {
    let current_path = Path::new(".");
    if let Err(_) = check_symlink(current_path) {
        println!("Something went wrong! Symlinks are not supported yet");
        process::exit(EXIT_CANNOT_EXECUTE);
    }

    let metadata = match current_path.metadata() {
        Ok(meta) => meta,
        Err(e) => {
            println!("Something went wrong! {}", e);
            process::exit(EXIT_CANNOT_EXECUTE);
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

