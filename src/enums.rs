use std::fmt;
use std::str::FromStr;

#[allow(dead_code)]
#[repr(u8)]
pub enum StatusCodes {
    Success = 0,
}

#[derive(Debug)]
pub enum BuiltInCommand {
    Exit(i32),
    Echo(String),
}

impl fmt::Display for BuiltInCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltInCommand::Exit(c) => write!(f, "exit {c}"),
            BuiltInCommand::Echo(s) => write!(f, "echo {s}"),
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
            ["echo", message @ ..] => Ok(BuiltInCommand::Echo(message.join(" "))),
            _ => Err(()),
        }
    }
}
