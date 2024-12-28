use crate::shell::Shell;
use crate::Command;
use std::env;

pub struct CAT;

impl Command for CAT {
    fn execute(&self, _shell: &Shell, args: &[String]) {
        match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(e) => eprintln!("Error getting current directory: {}", e),
        }
    }

    fn name(&self) -> &str {
        "cat"
    }
}
