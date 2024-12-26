#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        if input.starts_with("exit") {
            break;
        }
        if !input.starts_with("invalid") {
            println!("{}", input);
        } else {
            println!("{}: command not found", input);
        }
    }
}
