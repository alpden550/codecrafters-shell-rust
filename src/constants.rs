use lazy_static::lazy_static;

lazy_static! {
    pub static ref BUILTINS: Vec<&'static str> = vec!["exit", "echo", "type", "pwd", "cd"];
}
