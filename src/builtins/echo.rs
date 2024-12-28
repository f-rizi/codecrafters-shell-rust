use crate::Command;

use crate::shell::Shell;

pub struct Echo;

impl Command for Echo {
    fn execute(&self, _shell: &Shell, args: &[String]) {
        println!("{}", args.join(" "));
    }

    fn name(&self) -> &str {
        "echo"
    }
}
