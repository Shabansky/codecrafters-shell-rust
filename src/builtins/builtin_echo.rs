use crate::Expression;
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::expression::Expression;

    #[test]
    fn echo_returns_own_input() {
        let input = String::from("echo my test input");
        let expression = Expression::from(input);
        let expected_output = expression.arguments.join(" ").clone();
        let output = BuiltinEcho::from(expression).run();
        assert_eq!(expected_output, output);
    }
}
