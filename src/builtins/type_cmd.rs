use crate::Command;

use crate::shell::Shell;

pub struct TypeCmd;

impl Command for TypeCmd {
    fn execute(&self, shell: &Shell, args: &[String]) {
        if args.is_empty() {
            println!("type: missing argument");
            return;
        }

        let cmd = &args[0];
        if shell.is_builtin(cmd) {
            println!("{} is a shell builtin", cmd);
            return;
        }
        if let Some(path) = shell.is_external(cmd) {
            println!("{} is {}", cmd, path);
            return;
        } else {
            println!("{}: not found", cmd);
        }
    }

    fn name(&self) -> &str {
        "type"
    }
}
