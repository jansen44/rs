pub mod args;

use std::{fmt, env};
pub use args::Arg;

trait CowmandVector {
    fn try_insert_uniq(&mut self, wrapped_name: Option<String>);
}

impl CowmandVector for Vec<String> {
    fn try_insert_uniq(&mut self, wrapped_name: Option<String>) {
        if let Some(name) = wrapped_name {
            let possible_found = self.iter().find(|&item| item == &name);
            if possible_found == None { self.push(name); }
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Cowmand<'cow> {
    name: &'cow str,
    description: Option<&'cow str>,
    args: Vec<Arg<'cow>>,
    pub active_flags: Option<Vec<String>>,
    pub active_args: Option<Vec<String>>
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
                message.push_str(&format!(" -{},", s));
            }
            if let Some(l) = a.long_command {
                message.push_str(&format!(" --{},", l));
            }
            message.push_str(&format!("\t {}", a.name));
            if let Some(h) = a.help {
                message.push_str(&format!("\t- {}", h));
            }
            message.push_str("\n");
        }
        write!(f, "{}", message)
    }
}

impl<'cow> Cowmand<'cow> {
    pub fn new(name: &'cow str) -> Self {
        Cowmand { name, ..Default::default() }
    }

    pub fn description(mut self, description: &'cow str) -> Self {
        self.description = Some(description); self
    }

    pub fn arg(mut self, arg: Arg<'cow>) -> Self {
        self.args.push(arg); self
    }

    pub fn get_args(mut self) -> Self {
        let mut env_args = env::args();
        let mut flags = Vec::<String>::new();
        let mut args = Vec::<String>::new();
        if let None = env_args.next() { return self; }

        for arg in env_args {
            if Some(0) == arg.find("--") && arg.len() > 2 {
                let flag = String::from(arg).split_off(2);
                let possible_arg = self.find_by_long_command(&flag);
                flags.try_insert_uniq(possible_arg);
                continue;
            } 
            if Some(0) == arg.find('-') && arg.len() > 1 {
                let mut arg_iter = arg.chars();
                arg_iter.next();
                for flag in arg_iter {
                    let possible_arg = self.find_by_short_command(flag);
                    flags.try_insert_uniq(possible_arg);
                }
                continue;
            }
            args.try_insert_uniq(Some(String::from(arg)));
        }
        self.active_args = if args.len() > 0 { Some(args) } else { None };
        self.active_flags = if flags.len() > 0 { Some(flags) } else { None };
        self
    }

    fn find_by_short_command(&self, command: char) -> Option<String> {
        let wrapped_arg = &self.args.iter().find(|&arg| {
            return arg.short_command != None 
                && arg.short_command.unwrap() == command;
        });
        if let Some(arg) = wrapped_arg { 
            return Some(String::from(arg.name)); 
        }
        None
    }

    fn find_by_long_command(&self, command: &'cow str) -> Option<String> {
        let wrapped_arg = &self.args.iter().find(|&arg| {
            return arg.long_command != None 
                && arg.long_command.unwrap() == command;
        });
        if let Some(arg) = wrapped_arg { 
            return Some(String::from(arg.name));
        }
        None
    }
}

