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
    match command {
        _ => {
            println!("{}: command not found", command);
        }
    }
}
