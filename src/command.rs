use std::process;

#[derive(Clone, Debug, Default)]
pub enum Type {
    Echo,
    Exit,
    Type,
    Invalid,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default)]
pub enum BuildType {
    Builtin,
    NoFound,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default)]
pub struct Command {
    pub command_type: Type,
    pub build_type: BuildType,
    pub parameters: Vec<String>,
}

impl Command {
    pub fn new(parts: &Vec<String>) -> Self {
        if parts.len() == 0 {
            return Command::default();
        }

        let cmd = parts[0].to_lowercase();

        if cmd.starts_with("invalid") {
            return Self {
                command_type: Type::Invalid,
                build_type: BuildType::NoFound,
                parameters: vec![cmd],
            };
        }

        let mut result = Command::default();

        match cmd.as_str() {
            "type" => {
                if parts.len() < 2 {
                    return result;
                }

                result.command_type = Type::Type;
                result.build_type = BuildType::Builtin;
                result.parameters = parts[1..].to_vec();
            }
            "echo" => {
                if parts.len() < 2 {
                    return result;
                }

                result.command_type = Type::Echo;
                result.build_type = BuildType::Builtin;
                result.parameters = parts[1..].to_vec();
            }
            "exit" => {
                if parts.len() < 2 {
                    return result;
                }

                result.command_type = Type::Exit;
                result.build_type = BuildType::Builtin;
                result.parameters = parts[1..].to_vec();
            }
            _ => {
                return result;
            }
        }
        result
    }

    pub fn execute(&self) {
        match self.command_type {
            Type::Echo => {
                for (index, item) in self.parameters.iter().enumerate() {
                    print!("{}", item);

                    if index != self.parameters.len() - 1 {
                        print!(" ");
                    }
                }
            }
            Type::Exit => {
                // process::exit(1);
                process::abort();
            }
            Type::Invalid => {
                print!("{}: command not found", self.parameters[0]);
            }
            Type::Type => match self.build_type {
                BuildType::Builtin => {
                    print!("{} is a shell builtin", self.parameters[0]);
                }
                BuildType::NoFound => {
                    print!("{}: not found", self.parameters[0])
                }
                _ => {}
            },
            Type::Unknown => {}
        }
        println!();
    }
}
