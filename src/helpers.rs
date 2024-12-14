use shellwords::split;

pub fn parse_shell_words(text: &str) -> Vec<String> {
    let words = split(text).unwrap_or_else(|e| {
        eprintln!("Error parsing shell words: {}", e);
        vec![]
    });

    words
}
