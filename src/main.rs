mod enums;

use enums::BuiltInCommand;
use std::io::{self, Write};

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
        Err(_) => println!("{}: command not found", command),
    }
}
