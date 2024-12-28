use crate::shell::Shell;
use crate::Command;

pub struct Echo;

impl Command for Echo {
    fn execute(&self, shell: &Shell, args: &[String]) {
        let result = args.join(" ").to_string();
        shell.write_output(&result.as_str());
    }

    fn name(&self) -> &str {
        "echo"
    }
}
