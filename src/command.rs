enum Command {
    Echo { message: String },
    Invalid,
    Exit { message: String },
    Unknown(Vec<String>),
}

impl Command {
    fn parse(resp: &[String]) -> Self {
        let cmd = resp[0].to_lowercase();

        match cmd.as_str() {
            "echo" => {
                if resp.len() != 2 {
                    return Command::Unknown(resp.to_vec());
                }

                let message = resp[1].clone();
                Command::Echo { message }
            }
            "exit" => {
                if resp.len() != 2 {
                    return Command::Unknown(resp.to_vec());
                }

                let message = resp[1].clone();
                Command::Exit { message }
            }
            "invalid" => {
                if resp.len() == 1 {
                    return Command::Invalid;
                } else {
                    return Command::Unknown(resp.to_vec());
                }
            }
            _ => {
                return Command::Unknown(resp.to_vec());
            }
        }
    }
}
