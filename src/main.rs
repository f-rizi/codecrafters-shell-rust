mod builtins;
mod command;
mod shell;

use command::Command;
use shell::Shell;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let path_env = std::env::var("PATH").unwrap_or_else(|_| String::from(""));
    let paths: Vec<String> = path_env.split(':').map(String::from).collect();
    let shell = Shell::new(paths);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<String> = input.split_whitespace().map(String::from).collect();
        let cmd = &parts[0];
        let args = &parts[1..];
        shell.execute_command(cmd, args);
    }
}
