use std::process;

use crate::shell::Shell;
use crate::Command;

pub struct Exit;

impl Command for Exit {
    fn execute(&self, _shell: &Shell, args: &[String]) {
        process::exit(0);
    }

    fn name(&self) -> &str {
        "exit"
    }
}