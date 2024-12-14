mod constants;
mod enums;
mod helpers;

use crate::helpers::{check_executable, execute_external_command};
use enums::BuiltInCommand;
use std::path::PathBuf;
use std::{
    env,
    io::{self, Write},
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
    match command.parse::<BuiltInCommand>() {
        Ok(BuiltInCommand::Exit(code)) => std::process::exit(code),
        Ok(BuiltInCommand::Echo(message)) => println!("{}", message),
        Ok(BuiltInCommand::Type(c)) => {
            if constants::BUILTINS.contains(&c.as_str()) {
                println!("{} is a shell builtin", c);
            } else {
                match check_executable(c.as_str()) {
                    Ok(p) => println!("{} is {}", c, p.display()),
                    Err(_) => eprintln!("{}: not found", c),
                }
            }
        }
        Ok(BuiltInCommand::Pwd) => {
            let path = env::current_dir().unwrap_or_else(|e| {
                eprintln!("Error getting current directory: {}", e);
                PathBuf::new()
            });
            println!("{}", path.display());
        }
        Ok(BuiltInCommand::Cd(path)) => {
            if path == "~" {
                match env::var("HOME") {
                    Ok(h) => env::set_current_dir(h).unwrap_or_else(|_| {
                        eprintln!("HOME not set");
                    }),
                    Err(_) => eprintln!("HOME not set"),
                }
                return;
            }

            env::set_current_dir(&path).unwrap_or_else(|_| {
                eprintln!("{}: No such file or directory", path);
            });
        }
        Err(_) => {
            if execute_external_command(command).is_err() {
                eprintln!("{}: command not found", command);
            }
        }
    }
}
