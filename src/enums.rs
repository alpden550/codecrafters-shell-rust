use std::fmt;
use std::str::FromStr;

use crate::helpers::parse_shell_words;

#[derive(Debug)]
pub enum Command {
    Exit(i32),
    Echo(String),
    Type(String),
    Pwd,
    Cd(String),
    External(String, Vec<String>),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Exit(c) => write!(f, "exit {c}"),
            Command::Echo(s) => write!(f, "echo {s}"),
            Command::Type(s) => write!(f, "type {s}"),
            Command::Pwd => write!(f, "pwd"),
            Command::Cd(s) => write!(f, "cd {s}"),
            Command::External(c, args) => {
                let args = args.join(" ");
                write!(f, "{c} {args}")
            }
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        let parts = parse_shell_words(command);
        if parts.is_empty() {
            return Err(());
        }
        let cmd = parts[0].as_str();
        let args = &parts[1..];

        match (cmd, args) {
            ("exit", [code]) => {
                let code = code.parse::<i32>().unwrap_or(0);
                Ok(Command::Exit(code))
            },
            ("exit", []) => Ok(Command::Exit(0)),
            ("echo", text) => Ok(Command::Echo(text.join(" "))),
            ("type", text) => Ok(Command::Type(text.join(" "))),
            ("pwd", []) => Ok(Command::Pwd),
            ("cd", [path]) => Ok(Command::Cd(path.to_string())),
            _ => Ok(Command::External(cmd.to_string(), Vec::from(args))),
        }
    }
}
