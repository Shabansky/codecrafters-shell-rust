use std::{io::Read, process};
struct BuiltinExit {
    code: i32,
}

impl BuiltinExit {
    fn from(expression: &Expression) -> Self {
        Self { code: 0 }
    }

    fn run(&self) {
        process::exit(self.code);
    }
}

#[derive(Debug)]
pub struct Expression {
    input: String,
    command: String,
    arguments: Vec<String>,
}

impl Expression {
    fn from(input: String) -> Self {
        let mut arguments: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        let command = arguments.pop().expect("input string must not be empty");
        Self {
            input,
            command,
            arguments,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ReadError {
    NoCommand,
}

pub fn read(input: String) -> Result<Expression, ReadError> {
    let input = input.trim();
    if input.len() == 0 {
        return Err(ReadError::NoCommand);
    }

    Ok(Expression::from(input.to_string()))
}

pub fn eval(expression: Expression) -> String {
    //TODO: Below needs to be split and create BuiltinExit properly
    if expression.input == "exit 0" {
        BuiltinExit::from(&expression).run();
    }
    format!("{}: command not found", expression.input.trim())
}

pub fn print(output: String) {
    println!("{}", output);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eval_returns_not_found() {
        let command = String::from("test");
        let expression = Expression::from(command.clone());

        assert_eq!(format!("{command}: command not found"), eval(expression));
    }

    #[test]
    fn empty_command_returns_read_error() {
        let command = String::from("       ");
        let expression = read(command);
        assert_eq!(ReadError::NoCommand, expression.unwrap_err());
    }
}
