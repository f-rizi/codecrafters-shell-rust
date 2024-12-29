use crate::shell::Shell;
use crate::Command;
use std::process;

pub struct Exit;

impl Command for Exit {
    fn execute(&self, _shell: &Shell, _args: &[String], append_output: bool, append_error: bool) {
        process::exit(0);
    }

    fn name(&self) -> &str {
        "exit"
    }
}
