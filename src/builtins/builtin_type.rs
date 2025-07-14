use super::get_builtins;
use crate::Expression;
pub struct BuiltinType {
    commands: Vec<String>,
}

impl BuiltinType {
    pub fn from(expression: Expression) -> Self {
        Self {
            commands: expression.arguments,
        }
    }

    pub fn run(self) -> String {
        let mut output = String::new();
        for command in self.commands {
            let is_builtin = get_builtins().contains(&command.as_str());
            if is_builtin {
                output.push_str(format!("{command} is a shell builtin\n").as_str());
            } else {
                output.push_str(format!("{command}: not found\n").as_str())
            }
        }

        output
            .strip_suffix("\n")
            .expect("output must have a newline suffix")
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn run_command_on_input(input: String) -> String {
        let expression = Expression::from(input);
        BuiltinType::from(expression).run()
    }

    #[test]
    fn type_returns_single_line_for_single_command() {
        let output = run_command_on_input(String::from("type exit"));

        assert_eq!("exit is a shell builtin", output);
    }

    #[test]
    fn type_returns_multiple_lines_for_multiple_commands() {
        let output = run_command_on_input(String::from("type exit me please"));
        println!("OUTPUT: {output}");
        assert_eq!(
            "exit is a shell builtin\nme: not found\nplease: not found",
            output
        );
    }
}
