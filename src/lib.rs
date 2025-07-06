pub struct Expression {
    input: String,
}

pub fn read(input: String) -> Expression {
    Expression { input }
}

pub fn eval(expression: Expression) -> String {
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
        let command = "test";

        let expression = Expression {
            input: command.to_string(),
        };

        assert_eq!(format!("{command}: command not found"), eval(expression));
    }
}
