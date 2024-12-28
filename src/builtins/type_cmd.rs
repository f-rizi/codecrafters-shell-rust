use crate::shell::Shell;
use crate::Command;

pub struct TypeCmd;

impl Command for TypeCmd {
    fn execute(&self, shell: &Shell, args: &[String]) {
        if args.is_empty() {
            return;
        }

        let cmd = &args[0];
        if shell.is_builtin(cmd) {
            let message = format!("{} is a shell builtin", cmd);
            shell.write_output(&message);
            return;
        }
        if let Some(path) = shell.is_external(cmd) {
            let message = format!("{} is {}", cmd, path);
            shell.write_output(&message);
            return;
        } else {
            let message = format!("{}: not found", cmd);
            //shell.write_error(&message);
        }
    }

    fn name(&self) -> &str {
        "type"
    }
}
