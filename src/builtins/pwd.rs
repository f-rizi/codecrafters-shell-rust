use crate::shell::Shell;
use crate::Command;
use std::env;

pub struct PWD;

impl Command for PWD {
    fn execute(&self, shell: &Shell, args: &[String]) {
        match env::current_dir() {
            Ok(path) => {
                let message = path.display();
                shell.write_output(&message.to_string());
            }
            Err(e) => {}
        }
    }

    fn name(&self) -> &str {
        "pwd"
    }
}
