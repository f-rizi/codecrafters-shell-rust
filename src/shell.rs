use crate::error_output::ErrorOutput;
use crate::output::Output;
use crate::{builtins, command::Command};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command as SysCommand;

pub struct Shell {
    pub paths: Vec<String>,
    pub builtins: HashMap<String, Box<dyn Command>>,
    pub output: Output,
    pub error_output: ErrorOutput,
}

impl Shell {
    pub fn new(paths: Vec<String>) -> Self {
        let mut builtins: HashMap<String, Box<dyn Command>> = HashMap::new();
        builtins.insert("echo".to_string(), Box::new(builtins::echo::Echo));
        builtins.insert("exit".to_string(), Box::new(builtins::exit::Exit));
        builtins.insert("type".to_string(), Box::new(builtins::type_cmd::TypeCmd));
        builtins.insert("pwd".to_string(), Box::new(builtins::pwd::PWD));
        builtins.insert("cd".to_string(), Box::new(builtins::cd::CD));
        builtins.insert("ls".to_string(), Box::new(builtins::ls::LS));

        Self {
            paths,
            builtins,
            output: Output::Std,
            error_output: ErrorOutput::Std,
        }
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
        }
    }

    pub fn execute_external(&self, cmd: &str, path: &String, args: &[String]) {
        for arg in args {
            let temp_args = vec![arg];
            let output = SysCommand::new(&path).args(temp_args).output();

            match output {
                Ok(output) => {
                    if !output.status.success() {
                        let mut error_message = String::from_utf8_lossy(&output.stderr).to_string();

                        let path_prefix = format!("{}:", path);
                        let cmd_prefix = format!("{}:", cmd);

                        if error_message.starts_with(&path_prefix) {
                            error_message = error_message.replacen(&path_prefix, &cmd_prefix, 1);
                        }

                        self.write_error(&error_message.trim());
                    } else {
                        let output_message = String::from_utf8_lossy(&output.stdout);
                        self.write_output(&output_message.trim());
                    }
                }
                Err(e) => {
                    self.write_error(&format!("Failed to execute command: {}", e));
                }
            }
        }
    }

    pub fn write_output(&self, message: &str) {
        match &self.output {
            Output::Std => {
                println!("{}", message);
            }
            Output::File(file_path) => {
                let has_content = Path::new(file_path)
                    .metadata()
                    .map(|m| m.len() > 0)
                    .unwrap_or(false);

                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(file_path)
                {
                    if has_content {
                        if let Err(e) = writeln!(file, "{}", message) {
                            //eprintln!("Failed to write to error file {}: {}", file_path, e);
                        }
                    } else {
                        if let Err(e) = write!(file, "{}", message) {
                            //eprintln!("Failed to write to error file {}: {}", file_path, e);
                        }
                    }
                } else {
                    //eprintln!("Failed to open error file: {}", file_path);
                }
            }
        }
    }

    pub fn write_error(&self, message: &str) {
        match &self.error_output {
            ErrorOutput::Std => {
                println!("{}", message);
            }
            ErrorOutput::File(file_path) => {
                let has_content = Path::new(file_path)
                    .metadata()
                    .map(|m| m.len() > 0)
                    .unwrap_or(false);

                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(file_path)
                {
                    if has_content {
                        if let Err(e) = write!(file, "{}", message) {
                            //eprintln!("Failed to write to error file {}: {}", file_path, e);
                        }
                    } else {
                        if let Err(e) = write!(file, "{}", message) {
                            //eprintln!("Failed to write to error file {}: {}", file_path, e);
                        }
                    }
                } else {
                    //eprintln!("Failed to open error file: {}", file_path);
                }
            }
        }
    }

    pub fn get_current_dir(&self) -> std::path::PathBuf {
        match std::env::current_dir() {
            Ok(dir) => dir,
            Err(_) => std::path::PathBuf::from("."),
        }
    }

    pub fn resolve_path(&self, path: &str) -> std::path::PathBuf {
        let p = std::path::Path::new(path);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            self.get_current_dir().join(p)
        }
    }
}
