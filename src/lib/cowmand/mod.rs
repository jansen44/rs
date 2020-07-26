pub mod args;

use std::fmt;
pub use args::Arg;

#[derive(Default)]
pub struct Cowmand<'cow> {
    name: &'cow str,
    description: Option<&'cow str>,
    args: Vec<Arg<'cow>>
}

impl<'cow> fmt::Display for Cowmand<'cow> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut message = String::from(self.name);
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

impl<'cow> Cowmand<'cow> {
    pub fn new(name: &'cow str) -> Self {
        Cowmand {
            name: name,
            ..Default::default()
        }
    }

    pub fn description(mut self, description: &'cow str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn arg(mut self, arg: Arg<'static>) -> Self {
        self.args.push(arg);
        self
    }
}

