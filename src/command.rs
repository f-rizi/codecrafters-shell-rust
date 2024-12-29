use crate::shell::Shell;

pub trait Command {
    fn execute(&self, shell: &Shell, args: &[String], append_output: bool, append_error: bool);
    fn name(&self) -> &str;
}
