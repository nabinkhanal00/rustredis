pub enum Command {
    Ping,
    Echo(String),
    Get(String),
    Set(String, String),
}

impl TryFrom<Result<Primitive, CommandError>> for Command {
    type Error = CommandError;

    fn try_from(value: Result<Primitive, CommandError>) -> Result<Self, Self::Error> {
        value?.try_into()
    }
}

impl TryFrom<Primitive> for Command {
    type Error = CommandError;
    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        let value = if let Primitive::Array(value) = value.get_error()? {
            value
        } else {
            return Err(CommandError::InvalidCommandFormat);
        };

        let mut value = value.into_iter();
        let cmd = if let Some(cmd) = value.next() {
            if let Primitive::String(cmd) = cmd.get_error()? {
                cmd
            } else {
                return Err(CommandError::InvalidCommandFormat);
            }
        } else {
            return Err(CommandError::InvalidCommandFormat);
        };

        let result = {
            let mut next_arg = || match value.next() {
                Some(Primitive::String(s)) => Ok(s),
                Some(arg) => Err(CommandError::NonStringArgument(arg)),
                None => Err(CommandError::NotEnoughArguments),
            };
            match cmd.to_uppercase().as_str() {
                "PING" => Command::Ping,
                "ECHO" => Command::Echo(next_arg()?),
                "GET" => Command::Get(next_arg()?),
                "SET" => Command::Set(next_arg()?, next_arg()?),
                _ => {
                    return Err(CommandError::CommandNotImplemented(cmd));
                }
            }
        };
        if value.next().is_some() {
            return Err(CommandError::TooManyArguments);
        };
        Ok(result)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Primitive {
    Integer(i64),
    String(String),
    Error(String),
    Array(Vec<Primitive>),
}

impl Primitive {
    fn get_error(self) -> Result<Primitive, CommandError> {
        if let Primitive::Error(err) = self {
            Err(CommandError::RedisError(err))
        } else {
            Ok(self)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandError {
    UnexpectedEndOfFile,
    UnexpectedCharacter(char),
    NegativeSize(i64),
    RedisError(String),
    InvalidCommandFormat,
    CommandNotImplemented(String),
    NonStringArgument(Primitive),
    NotEnoughArguments,
    TooManyArguments,
}
