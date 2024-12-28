use std::{env, process};

use crate::shell::Shell;
use crate::Command;

pub struct PWD;

impl Command for PWD {
    fn execute(&self, _shell: &Shell, args: &[String]) {
        match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(e) => eprintln!("Error getting current directory: {}", e),
        }
    }

    fn name(&self) -> &str {
        "pwd"
    }
}
