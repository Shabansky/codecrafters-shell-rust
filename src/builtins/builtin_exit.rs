use crate::Expression;
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

#[cfg(test)]
mod test {
    use super::*;

    fn return_builtin_on_input(input: String) -> BuiltinExit {
        let expression = Expression::from(input);
        BuiltinExit::from(expression)
    }

    #[test]
    fn no_exit_code_returns_zero() {
        let input = String::from("exit");
        let builtin = return_builtin_on_input(input);

        assert_eq!(0, builtin.code);
    }

    #[test]
    fn zero_exit_code_returns_zero() {
        let input = String::from("exit 0");
        let builtin = return_builtin_on_input(input);

        assert_eq!(0, builtin.code);
    }

    #[test]
    fn negative_exit_code_returns_self() {
        let input = String::from("exit -1");
        let builtin = return_builtin_on_input(input);

        assert_eq!(-1, builtin.code);
    }

    #[test]
    fn positive_exit_code_returns_self() {
        let input = String::from("exit 1");
        let builtin = return_builtin_on_input(input);

        assert_eq!(1, builtin.code);
    }

    #[test]
    fn non_number_returns_zero() {
        let input = String::from("exit incorrect");
        let builtin = return_builtin_on_input(input);

        assert_eq!(0, builtin.code);
    }

    #[test]
    fn multiple_arguments_return_zero() {
        let input = String::from("exit something here");
        let builtin = return_builtin_on_input(input);

        assert_eq!(0, builtin.code);
    }

    #[test]
    fn multiple_arguments_with_numeral_return_zero() {
        let input = String::from("exit something 1");
        let builtin = return_builtin_on_input(input);

        assert_eq!(0, builtin.code);
    }
}
