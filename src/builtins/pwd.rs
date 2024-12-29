use crate::shell::Shell;
use crate::Command;
use std::env;

pub struct PWD;

impl Command for PWD {
    fn execute(&self, shell: &Shell, args: &[String], append_output: bool, append_error: bool) {
        match env::current_dir() {
            Ok(path) => {
                let message = path.display().to_string() + "\n";
                shell.write_output(&message.to_string(), append_output);
            }
            Err(e) => {}
        }
    }

    fn name(&self) -> &str {
        "pwd"
    }
}
