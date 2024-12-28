use crate::shell::Shell;
use crate::Command;
use std::fs;

pub struct LS;

impl Command for LS {
    fn execute(&self, shell: &Shell, args: &[String]) {
        let dirs_to_list = if args.is_empty() {
            vec![shell.get_current_dir()]
        } else {
            args.iter().map(|dir| shell.resolve_path(dir)).collect()
        };

        for (i, dir) in dirs_to_list.iter().enumerate() {
            match fs::read_dir(dir) {
                Ok(entries) => {
                    let mut files: Vec<fs::DirEntry> = entries.filter_map(Result::ok).collect();
                    files.sort_by_key(|e| e.file_name().to_str().unwrap().to_string());

                    for entry in files {
                        let message = entry.file_name().to_str().unwrap().to_string() + "\n";
                        shell.write_output(&message);
                    }
                }
                Err(e) => {
                    //shell.write_error(&format!("ls: cannot access '{}': {}", dir.display(), e));
                }
            }
        }
    }

    fn name(&self) -> &str {
        "ls"
    }
}
