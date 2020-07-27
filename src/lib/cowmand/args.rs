#[derive(Default)]
pub struct Arg<'arg> {
    pub name: &'arg str,
    pub short_command: Option<char>,
    pub help: Option<&'arg str>,
    pub long_command: Option<&'arg str>
}

impl<'arg> Arg<'arg> {
    pub fn new(name: &'arg str) -> Self {
        Arg {
            name: name,
            ..Default::default()
        }
    }

    pub fn short_command(mut self, short: char) -> Self {
        self.short_command = Some(short);
        self
    }

    pub fn long_command(mut self, long: &'arg str) -> Self {
        self.long_command = Some(long);
        self
    }

    pub fn help(mut self, help: &'arg str) -> Self {
        self.help = Some(help);
        self
    }
}

