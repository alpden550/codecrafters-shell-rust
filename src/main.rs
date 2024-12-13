mod constants;
mod enums;

use enums::BuiltInCommand;
use std::path::PathBuf;
use std::{
    fs,
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
    if execute_external_command(command).is_ok()  {
        return;
    }

    match command.parse::<BuiltInCommand>() {
        Ok(BuiltInCommand::Exit(code)) => std::process::exit(code),
        Ok(BuiltInCommand::Echo(message)) => println!("{}", message),
        Ok(BuiltInCommand::Type(c)) => {
            if constants::BUILTINS.contains(&c.as_str()) {
                println!("{} is a shell builtin", c);
            } else {
                match check_executable(c.as_str()) {
                    Ok(p) => println!("{} is {}", c, p.display()),
                    Err(_) => println!("{}: not found", c),
                }
            }
        }
        Err(_) => println!("{}: not found", command),
    }
}

fn execute_external_command(args: &str) -> Result<(), String> {
    let mut parts = args.split_whitespace();
    let command = parts.next().unwrap_or("");
    let arguments: Vec<&str> = parts.collect();

    match check_executable(command) {
        Ok(path) => {
            let status = std::process::Command::new(path)
                .args(arguments)
                .status()
                .expect("Failed to execute command");

            if !status.success() {
                eprintln!("{}: command failed with status {}", command, status);
            }
            Ok(())
        }
        Err(_) => Err("command not found".to_string()),
    }
}

fn check_executable(command: &str) -> Result<PathBuf, String> {
    let env_path = std::env::var("PATH").map_err(|_| "Failed to read PATH environment variable")?;

    env_path
        .split(':')
        .filter_map(|dir| {
            let path = Path::new(dir).join(command);
            if fs::metadata(&path)
                .map(|metadata| metadata.is_file())
                .unwrap_or(false)
            {
                Some(path)
            } else {
                None
            }
        })
        .next()
        .ok_or_else(|| format!("{}: command not found", command))
}
