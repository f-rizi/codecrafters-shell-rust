mod command;

#[allow(unused_imports)]
use std::io::{self, Write};

use command::Command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        let parts: Vec<String> = input.split_whitespace().map(String::from).collect();
        let command: Command = Command::parse(&parts);

        match command {
            Command::Echo { message } => {
                for (index, item) in message.iter().enumerate() {
                    print!("{}", item);

                    if index != message.len() - 1 {
                        print!(" ");
                    }
                }
            }
            Command::Exit { message } => {
                break;
            }
            Command::Invalid => {
                print!("{}: command not found", input);
            }
            _ => {}
        }

        println!();
    }
}
