use crate::shell::Shell;
use crate::Command;
use std::env;

pub struct CD;

impl Command for CD {
    fn execute(&self, _shell: &Shell, args: &[String], append_output: bool, append_error: bool) {
        if args.is_empty() {
            println!("type: missing argument");
            return;
        }

        let path = &args[0];

        if path == "~" {
            let home_path = env::var("HOME").ok();

            if let Some(home) = home_path {
                self.go_to_directory(&home);
            }
        } else {
            self.go_to_directory(&path);
        }
    }

    fn name(&self) -> &str {
        "pwd"
    }
}

impl CD {
    fn go_to_directory(&self, path: &String) {
        match env::set_current_dir(&path) {
            Ok(_) => {}
            Err(_) => {
                println!("cd: {}: No such file or directory", path);
            }
        }
    }
}
