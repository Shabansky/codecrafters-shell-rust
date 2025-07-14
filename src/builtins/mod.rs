pub fn get_builtins() -> Vec<&'static str> {
    vec!["echo", "exit", "type"]
}

pub mod builtin_echo;
pub mod builtin_exit;
pub mod builtin_type;
