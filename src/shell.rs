use crate::{builtins, command::Command};
use std::collections::HashMap;
use std::process::Command as SysCommand;

pub struct Shell {
    pub paths: Vec<String>,
    pub builtins: HashMap<String, Box<dyn Command>>,
}

impl Shell {
    pub fn new(paths: Vec<String>) -> Self {
        let mut builtins: HashMap<String, Box<dyn Command>> = HashMap::new();
        builtins.insert("echo".to_string(), Box::new(builtins::echo::Echo));
        builtins.insert("exit".to_string(), Box::new(builtins::exit::Exit));
        builtins.insert("type".to_string(), Box::new(builtins::type_cmd::TypeCmd));
        builtins.insert("pwd".to_string(), Box::new(builtins::pwd::PWD));
        builtins.insert("cd".to_string(), Box::new(builtins::cd::CD));

        Self { paths, builtins }
    }

    pub fn is_builtin(&self, cmd: &str) -> bool {
        self.builtins.contains_key(cmd)
    }

    pub fn is_external(&self, cmd: &str) -> Option<String> {
        for path in &self.paths {
            let cmd_path = format!("{}/{}", path, cmd);
            if std::fs::metadata(&cmd_path).is_ok() {
                return Some(cmd_path);
            }
        }

        return None;
    }

    pub fn execute_command(&self, cmd: &str, args: &[String]) {
        if let Some(command) = self.builtins.get(cmd) {
            command.execute(self, args);
        } else if let Some(path) = self.is_external(cmd) {
            self.execute_external(cmd, &path, &args);
        } else {
            println!("{}: not found", cmd);
        }
    }

    pub fn execute_external(&self, cmd: &str, path: &String, args: &[String]) {
        let status = SysCommand::new(&path).args(args).status();

        match status {
            Ok(status) => {
                if !status.success() {
                    eprintln!("{} exited with status {}", cmd, status);
                }
            }
            Err(e) => {
                eprintln!("Failed to execute {}: {}", cmd, e);
            }
        }
    }
}
