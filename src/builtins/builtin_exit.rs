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
