use shellwords::split;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn parse_shell_words(text: &str) -> Vec<String> {
    let words = split(text).unwrap_or_else(|e| {
        eprintln!("Error parsing shell words: {}", e);
        vec![]
    });

    words
}

pub fn check_executable(command: &str) -> Result<PathBuf, String> {
    let env_path = env::var("PATH").map_err(|_| "Failed to read PATH environment variable")?;

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

pub fn execute_external_command(cmd: &str, args: Vec<String>) -> Result<(), String> {
    match check_executable(cmd) {
        Ok(path) => {
            let status = std::process::Command::new(path)
                .args(args)
                .status()
                .expect("Failed to execute command");

            if !status.success() {
                eprintln!("{}: command failed with status {}", cmd, status);
            }
            Ok(())
        }
        Err(_) => Err("command not found".to_string()),
    }
}
