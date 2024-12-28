use crate::shell::Shell;
use crate::Command;
use std::env;

pub struct CD;

impl Command for CD {
    fn execute(&self, _shell: &Shell, args: &[String]) {
        if args.is_empty() {
            println!("type: missing argument");
            return;
        }

        let path = &args[0];

        match env::set_current_dir(&path) {
            Ok(_) => {}
            Err(_) => {
                println!("cd: {}: No such file or directory", path);
            }
        }
    }

    fn name(&self) -> &str {
        "pwd"
    }
}
