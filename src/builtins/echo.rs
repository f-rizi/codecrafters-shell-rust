use crate::shell::Shell;
use crate::Command;

pub struct Echo;

impl Command for Echo {
    fn execute(&self, _shell: &Shell, args: &[String]) {
        println!("{}", args.join(" "));
    }

    fn name(&self) -> &str {
        "echo"
    }
}
