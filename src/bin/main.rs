use rs::app::App;
use rs::cowmand::{Cowmand, Arg};

fn main() {
    let cow = Cowmand::new("rs")
        .description("A zero-dependency 'ls' alternative.")
        .arg(
            Arg::new()
                .short_command('l')
                .long_command("list")
                .help("show files/directories in list mode.")
        )
        .arg(
            Arg::new()
                .short_command('a')
                .long_command("all")
                .help("show all files/directories.")
        )
        .arg(
            Arg::new()
                .short_command('h')
                .long_command("help")
                .help("show this help message.")
        );
                
    println!("{}", cow);
    let app = App::new();
    app.run();
}

