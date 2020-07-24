use std::fmt;

// Todo: Change 'String' to &str wherever it's possible
#[derive(Default)]
pub struct Arg {
    short_command: Option<char>,
    help: Option<String>,
    long_command: Option<String>
}

impl Arg {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn short_command(mut self, short: char) -> Self {
        self.short_command = Some(short);
        self
    }

    pub fn long_command(mut self, long: String) -> Self {
        self.long_command = Some(long);
        self
    }

    pub fn help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }
}

// Todo: Change 'String' to &str wherever it's possible
#[derive(Default)]
pub struct Cowmand {
    name: String,
    description: Option<String>,
    args: Vec<Arg>
}

impl fmt::Display for Cowmand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut message = self.name.clone();
        if let Some(d) = &self.description {
            message.push_str(&format!(" - {}", d));
        }
        if self.args.len() > 0 { message.push_str("\n\n"); }

        for a in &self.args {
            if let Some(s) = a.short_command {
                message.push_str(&format!(" -{}", s));
            }
            if let Some(l) = &a.long_command {
                message.push_str(&format!(", --{}", l));
            }
            if let Some(h) = &a.help {
                message.push_str(&format!("\t\t {}", h));
            }
            message.push_str("\n");
        }

        write!(f, "{}\n", message)
    }
}

impl Cowmand {
    pub fn new(name: String) -> Self {
        Cowmand {
            name: name,
            ..Default::default()
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        self
    }
}

