use std::fmt;

// Todo: Change 'String' to &str wherever it's possible
pub struct Arg {
    description: String,
    short_command: Option<char>,
    long_command: Option<String>
}

// Todo: Change 'String' to &str wherever it's possible
pub struct Cowmand {
    name: String,
    description: Option<String>,
    args: Vec<Arg>
}

impl fmt::Display for Cowmand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.description {
            Some(d) => writeln!(f, "{} - {}", self.name, d),
            None => writeln!(f, "{}", self.name)
        }
    }
}

impl Cowmand {
    pub fn new(name: String) -> Self {
        Cowmand {
            name: name,
            description: None,
            args: Vec::new()
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

