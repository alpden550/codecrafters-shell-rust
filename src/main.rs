mod constants;
mod enums;

use enums::BuiltInCommand;
use std::{
    io::{self, Write},
    path::Path,
};

fn main() {
    repl();
}

fn repl() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        execute_command(input.trim());
    }
}

fn execute_command(command: &str) {
    let path = std::env::var("PATH").unwrap();

    match command.parse::<BuiltInCommand>() {
        Ok(BuiltInCommand::Exit(code)) => std::process::exit(code),
        Ok(BuiltInCommand::Echo(message)) => println!("{}", message),
        Ok(BuiltInCommand::Type(c)) => {
            if constants::BUILTINS.contains(&c.as_str()) {
                println!("{} is a shell builtin", c);
            } else {
                let found = path.split(":").find(|dir| Path::new(dir).join(&c).exists());
                match found {
                    Some(dir) => {
                        println!("{} is {}", c, Path::new(dir).join(&c).display());
                    }
                    None => println!("{}: not found", c),
                }
            }
        }
        Err(_) => println!("{}: command not found", command),
    }
}
