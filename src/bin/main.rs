use rs::{RsApp, Cowmand};

fn main() {
    let cow = Cowmand::new(String::from("rs"))
        .description(String::from("A zero-dependency 'ls' alternative."));

    println!("{}", cow);

    let app = RsApp::new();
    app.run();
}

