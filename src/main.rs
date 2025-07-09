use codecrafters_shell::{eval, print, read};
#[allow(unused_imports)]
use std::io::{self, Write};
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let expression = read(input);
        match expression {
            //TODO: Too general. Assumes all errors are treated the same way.
            Err(_) => {
                continue;
            }
            Ok(expression) => {
                let output = eval(expression);
                print(output);
            }
        }
    }
}
