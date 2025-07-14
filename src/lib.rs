mod builtins;
use builtins::{builtin_echo, builtin_exit, builtin_type};

mod expression;
use expression::Expression;

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
        "type" => Some(builtin_type::BuiltinType::from(expression).run()),
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
