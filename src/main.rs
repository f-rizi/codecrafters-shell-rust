mod builtins;
mod command;
mod error_output;
mod output;
mod shell;

use command::Command;
use shell::Shell;
use shell_words::split;
use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
};

fn main() {
    let path_env = std::env::var("PATH").unwrap_or_else(|_| String::from(""));
    let paths: Vec<String> = path_env.split(':').map(String::from).collect();
    let mut shell = Shell::new(paths);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts = match split(input) {
            Ok(parts) => parts,
            Err(e) => {
                continue;
            }
        };

        let cmd = &parts[0];
        let mut args: &[String] = &parts[1..];
        let mut args2: &[String] = &parts[1..];

        if args.contains(&String::from(">")) || args.contains(&String::from("1>")) {
            shell.output = output::Output::File(args.last().unwrap().clone());
            args = &args2[0..args.len() - 2];
        } else {
            shell.output = output::Output::Std;
        }

        if args.contains(&String::from(">>")) || args.contains(&String::from("2>")) {
            let file_path = args.last().unwrap().clone();
            let path = Path::new(&file_path);
            if let Some(parent) = path.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {

                } else {
                    let mut file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .append(true)
                        .open(path);
                }
            }

            shell.error_output = error_output::ErrorOutput::File(file_path);
            args = &args2[0..args.len() - 2];
        } else {
            shell.error_output = error_output::ErrorOutput::Std;
        }

        shell.execute_command(cmd, args);
    }
}
