use crate::shell::Shell;
use crate::Command;

pub struct TypeCmd;

impl Command for TypeCmd {
    fn execute(&self, shell: &Shell, args: &[String], append_output: bool, append_error: bool) {
        if args.is_empty() {
            return;
        }

        let cmd = &args[0];
        if shell.is_builtin(cmd) {
            let message = format!("{} is a shell builtin\n", cmd);
            shell.write_output(&message, append_output);
            return;
        }
        if let Some(path) = shell.is_external(cmd) {
            let message = format!("{} is {}\n", cmd, path);
            shell.write_output(&message, append_output);
            return;
        } else {
            let message = format!("{}: not found\n", cmd);
            shell.write_output(&message, append_output);
        }
    }

    fn name(&self) -> &str {
        "type"
    }
}
