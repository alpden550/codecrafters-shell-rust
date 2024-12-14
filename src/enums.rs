use std::fmt;
use std::str::FromStr;

use crate::helpers::parse_shell_words;

#[derive(Debug)]
pub enum BuiltInCommand {
    Exit(i32),
    Echo(String),
    Type(String),
    Pwd,
    Cd(String),
}

impl fmt::Display for BuiltInCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltInCommand::Exit(c) => write!(f, "exit {c}"),
            BuiltInCommand::Echo(s) => write!(f, "echo {s}"),
            BuiltInCommand::Type(s) => write!(f, "type {s}"),
            BuiltInCommand::Pwd => write!(f, "pwd"),
            BuiltInCommand::Cd(s) => write!(f, "cd {s}"),
        }
    }
}

impl FromStr for BuiltInCommand {
    type Err = ();

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts.as_slice() {
            ["exit", code] => {
                let code = code.parse::<i32>().unwrap_or(0);
                Ok(BuiltInCommand::Exit(code))
            }
            ["echo", text] => Ok(BuiltInCommand::Echo(parse_shell_words(text).join(" "))),
            ["type", text] => Ok(BuiltInCommand::Type(text.to_string())),
            ["pwd"] => Ok(BuiltInCommand::Pwd),
            ["cd", path] => Ok(BuiltInCommand::Cd(path.to_string())),
            _ => Err(()),
        }
    }
}
