use crate::shell::Shell;
use crate::Command;
use std::fs;

pub struct LS;

impl Command for LS {
    fn execute(&self, shell: &Shell, args: &[String], append_output: bool, append_error: bool) {
        let dirs_to_list = if args.is_empty() {
            vec![shell.get_current_dir()]
        } else {
            args.iter().map(|dir| shell.resolve_path(dir)).collect()
        };

        let mut final_message = String::from("");

        for (i, dir) in dirs_to_list.iter().enumerate() {
            match fs::read_dir(dir) {
                Ok(entries) => {
                    let mut files: Vec<fs::DirEntry> = entries.filter_map(Result::ok).collect();
                    files.sort_by_key(|e| e.file_name().to_str().unwrap().to_string());

                    for entry in files {
                        let message = entry.file_name().to_str().unwrap().to_string() + "\n";
                        final_message.push_str(&message);
                    }
                }
                Err(e) => {
                    let error_description = e
                        .to_string()
                        .split(" (")
                        .next()
                        .unwrap_or(&e.to_string())
                        .to_string();

                    let formatted_error = format!("ls: {}: {}\n", args[i], error_description);
                    shell.write_error(&formatted_error, append_error);
                    return;
                }
            }
        }

        shell.write_output(&final_message, append_output);
    }

    fn name(&self) -> &str {
        "ls"
    }
}
