use crate::output::format;
use crate::types::{Command, CommandError};

pub fn eval(commands: Result<Vec<Command>, CommandError>) -> String {
    match commands {
        Ok(commands) => {
            for command in commands {
                match command {
                    Command::Ping => return format(String::from("PONG")),
                    Command::Get(_) => todo!(),
                    Command::Set(_, _) => todo!(),
                    Command::Echo(value) => return format(value),
                };
            }
            String::from("")
        }
        Err(e) => format!("-ERR {:?},", e),
    }
}
