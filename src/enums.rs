use std::fmt;
use std::str::FromStr;

#[repr(u8)]
pub enum StatusCodes {
    Success(()) = 0,
}

pub enum BuiltInCommand {
    Exit,
}

impl fmt::Display for BuiltInCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltInCommand::Exit => write!(f, "exit 0"),
        }
    }
}

impl FromStr for BuiltInCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit 0" => Ok(BuiltInCommand::Exit),
            _ => Err(()),
        }
    }
}
