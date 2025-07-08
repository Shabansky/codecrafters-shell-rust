use std::process;
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

pub struct Expression {
    input: String,
}

pub fn read(input: String) -> Expression {
    //TODO: .to_string() is overkill. Consider input to be of type &str instead
    Expression {
        input: input.trim().to_string(),
    }
}

pub fn eval(expression: Expression) -> String {
    println!("{}", expression.input);
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
        let command = "test";

        let expression = Expression {
            input: command.to_string(),
        };

        assert_eq!(format!("{command}: command not found"), eval(expression));
    }
}
