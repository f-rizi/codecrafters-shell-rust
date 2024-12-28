use crate::shell::Shell;

pub trait Command {
    fn execute(&self, shell: &Shell, args: &[String]);
    fn name(&self) -> &str;
}
