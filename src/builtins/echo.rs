use crate::shell::Shell;
use crate::Command;

pub struct Echo;

impl Command for Echo {
    fn execute(&self, shell: &Shell, args: &[String], append_output: bool, append_error: bool) {
        let result = args.join(" ").to_string() + "\n";
        shell.write_output(&result.as_str(), append_output);
    }

    fn name(&self) -> &str {
        "echo"
    }
}
