#[derive(Clone, Debug)]
pub enum Command {
    Echo { message: Vec<String> },
    Invalid,
    Exit { message: String },
    Unknown(Vec<String>),
}

impl Command {
    pub fn parse(parts: &Vec<String>) -> Self {
        let cmd = parts[0].to_lowercase();

        if cmd.starts_with("invalid") {
            if parts.len() == 1 {
                return Command::Invalid;
            } else {
                return Command::Unknown(parts.to_vec());
            }
        }

        match cmd.as_str() {
            "echo" => {
                if parts.len() < 2 {
                    return Command::Unknown(parts.clone());
                }

                Command::Echo {
                    message: parts[1..].to_vec(),
                }
            }
            "exit" => {
                if parts.len() != 2 {
                    return Command::Unknown(parts.to_vec());
                }

                let message = parts[1].clone();
                Command::Exit { message }
            }
            _ => {
                return Command::Unknown(parts.to_vec());
            }
        }
    }
}
