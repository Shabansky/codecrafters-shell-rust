#[derive(Debug)]
pub struct Expression {
    pub command: String,
    pub arguments: Vec<String>,
}

impl Expression {
    pub fn from(input: String) -> Self {
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("").to_string();
        let arguments: Vec<String> = parts.map(|s| s.to_string()).collect();

        Self { command, arguments }
    }
}
