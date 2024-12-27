mod command;

#[allow(unused_imports)]
use std::io::{self, Write};

use command::{Command, Type};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        let parts: Vec<String> = input.split_whitespace().map(String::from).collect();
        let command: Command = Command::new(&parts);
        command.execute();
    }
}
