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
