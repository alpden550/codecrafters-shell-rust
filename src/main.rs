mod constants;
mod enums;
mod helpers;

use crate::helpers::{check_executable, execute_external_command};
use enums::Command;
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
    match command.parse::<Command>() {
        Ok(Command::Exit(code)) => std::process::exit(code),
        Ok(Command::Echo(message)) => println!("{}", message),
        Ok(Command::Type(c)) => {
            if constants::BUILTINS.contains(&c.as_str()) {
                println!("{} is a shell builtin", c);
            } else if let Ok(p) = check_executable(c.as_str()) {
                println!("{} is {}", c, p.display());
            } else {
                eprintln!("{}: not found", c);
            }
        }
        Ok(Command::Pwd) => {
            let path = env::current_dir().unwrap_or_else(|e| {
                eprintln!("Error getting current directory: {}", e);
                PathBuf::new()
            });
            println!("{}", path.display());
        }
        Ok(Command::Cd(path)) => {
            let expanded_path = if path.starts_with("~/") {
                match env::var("HOME") {
                    Ok(home) => PathBuf::from(home).join(&path[2..]),
                    Err(_) => {
                        eprintln!("cd: HOME not set");
                        return;
                    }
                }
            } else if path == "~" {
                match env::var("HOME") {
                    Ok(home) => PathBuf::from(home),
                    Err(_) => {
                        eprintln!("cd: HOME not set");
                        return;
                    }
                }
            } else {
                PathBuf::from(&path)
            };

            if env::set_current_dir(&expanded_path).is_err() {
                eprintln!("cd: {}: No such file or directory", expanded_path.display());
            }
        }
        Ok(Command::External(cmd, args)) => {
            if execute_external_command(cmd.as_str(), args.clone()).is_err() {
                eprintln!("{}: command not found", cmd);
            }
        }
        Err(_) => {
            eprintln!("{}: command not found", command);
        }
    }
}
