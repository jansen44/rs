use rs::{RsApp, Cowmand, Arg};

fn main() {
    let cow = Cowmand::new(String::from("rs"))
        .description(String::from("A zero-dependency 'ls' alternative."))
        .arg(
            Arg::new()
                .short_command('l')
                .long_command(String::from("list"))
                .help(String::from("show files/directories in list mode."))
        )
        .arg(
            Arg::new()
                .short_command('a')
                .long_command(String::from("all"))
                .help(String::from("show all files/directories."))
        )
        .arg(
            Arg::new()
                .short_command('h')
                .long_command(String::from("help"))
                .help(String::from("show this help message."))
        );
                
    println!("{}", cow);
    // let app = RsApp::new();
    // app.run();
}

