mod builtin_exit {
    use super::Expression;
    use std::process;

    type ExitCode = i32;
    pub struct BuiltinExit {
        code: ExitCode,
    }

    impl BuiltinExit {
        pub fn from(expression: Expression) -> Self {
            let code = expression
                .arguments
                .first()
                .and_then(|arg| arg.parse::<ExitCode>().ok())
                .unwrap_or(0);

            Self { code }
        }

        pub fn run(&self) {
            process::exit(self.code);
        }
    }
}

mod builtin_echo {
    use super::Expression;
    pub struct BuiltinEcho {
        text: String,
    }

    impl BuiltinEcho {
        pub fn from(expression: Expression) -> Self {
            let text = expression.arguments.join(" ");
            Self { text }
        }

        pub fn run(&self) -> String {
            format!("{}", self.text)
        }
    }
}

#[derive(Debug)]
pub struct Expression {
    command: String,
    arguments: Vec<String>,
}

impl Expression {
    fn from(input: String) -> Self {
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("").to_string();
        let arguments: Vec<String> = parts.map(|s| s.to_string()).collect();

        Self { command, arguments }
    }
}

#[derive(PartialEq, Debug)]
pub enum ReadError {
    NoCommand,
}

pub fn read(input: String) -> Result<Expression, ReadError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(ReadError::NoCommand);
    }

    Ok(Expression::from(input.to_string()))
}

pub fn eval(expression: Expression) -> Option<String> {
    let command = expression.command.clone();
    match expression.command.as_str() {
        "exit" => {
            builtin_exit::BuiltinExit::from(expression).run();
            None
        }
        "echo" => Some(builtin_echo::BuiltinEcho::from(expression).run()),
        _ => Some(format!("{command}: command not found")),
    }
}

pub fn print(output: String) {
    println!("{output}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eval_returns_not_found() {
        let command = String::from("test");
        let expression = Expression::from(command.clone());

        assert_eq!(
            format!("{command}: command not found"),
            eval(expression).unwrap()
        );
    }

    #[test]
    fn empty_command_returns_read_error() {
        let command = String::from("       ");
        let expression = read(command);
        assert_eq!(ReadError::NoCommand, expression.unwrap_err());
    }

    #[test]
    fn command_not_found_returns_first_segment() {
        let command = String::from("correct --false");
        let expression = Expression::from(command.clone());

        assert_eq!(
            format!("{}: command not found", "correct"),
            eval(expression).unwrap()
        )
    }
}
