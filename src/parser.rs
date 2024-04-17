use crate::types::{CommandError, Primitive};

pub struct Parser<I: Iterator<Item = char>>(pub I);

impl<I: Iterator<Item = char>> Parser<I> {
    fn parse_simple_string(&mut self) -> Result<String, CommandError> {
        let mut result = String::new();
        while let Some(c) = self.read_valid_char()? {
            result.push(c);
        }
        self.read_specific_char('\n')?;
        Ok(result)
    }
    fn read_valid_char(&mut self) -> Result<Option<char>, CommandError> {
        match self.0.next() {
            Some('\r') => Ok(None),
            Some('\n') => Err(CommandError::UnexpectedCharacter('\n')),
            Some(c) => Ok(Some(c)),
            None => Err(CommandError::UnexpectedEndOfFile),
        }
    }
    fn read_specific_char(&mut self, expected: char) -> Result<(), CommandError> {
        if let Some(c) = self.0.next() {
            if c == expected {
                Ok(())
            } else {
                Err(CommandError::UnexpectedCharacter(c))
            }
        } else {
            Err(CommandError::UnexpectedEndOfFile)
        }
    }

    fn read_existing_char(&mut self) -> Result<char, CommandError> {
        self.read_valid_char()?
            .ok_or(CommandError::UnexpectedCharacter('\r'))
    }

    fn parse_integer(&mut self) -> Result<i64, CommandError> {
        fn digit_from_char(c: char) -> Result<i64, CommandError> {
            c.to_digit(10)
                .map(|d| d as i64)
                .ok_or(CommandError::UnexpectedCharacter(c))
        }
        let (sign, mut result) = match self.read_existing_char()? {
            '-' => (-1, digit_from_char(self.read_existing_char()?)?),
            '+' => (1, digit_from_char(self.read_existing_char()?)?),
            c => (1, digit_from_char(c)?),
        };
        while let Some(c) = self.read_valid_char()? {
            result = result * 10 + digit_from_char(c)?;
        }
        self.read_specific_char('\n')?;
        Ok(result * sign)
    }
    fn parse_usize(&mut self) -> Result<usize, CommandError> {
        let number = self.parse_integer()?;
        if 0 <= number {
            Ok(number as usize)
        } else {
            Err(CommandError::NegativeSize(number))
        }
    }
    fn parse_bulk_string(&mut self) -> Result<String, CommandError> {
        let size = self.parse_usize()?;
        let mut result = String::with_capacity(size);
        for _ in 0..size {
            if let Some(c) = self.0.next() {
                result.push(c);
            } else {
                return Err(CommandError::UnexpectedEndOfFile);
            }
        }
        self.read_specific_char('\r')?;
        self.read_specific_char('\n')?;
        Ok(result)
    }
    fn parse_array(&mut self) -> Result<Vec<Primitive>, CommandError> {
        let size = self.parse_usize()?;
        let mut result = Vec::with_capacity(size);
        for _ in 0..size {
            if let Some(command) = self.next() {
                result.push(command?);
            } else {
                return Err(CommandError::UnexpectedEndOfFile);
            }
        }
        Ok(result)
    }
}

impl<I: Iterator<Item = char>> Iterator for Parser<I> {
    type Item = Result<Primitive, CommandError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next()? {
            '+' => Some(self.parse_simple_string().map(Primitive::String)),
            '-' => Some(self.parse_simple_string().map(Primitive::Error)),
            ':' => Some(self.parse_integer().map(Primitive::Integer)),
            '$' => Some(self.parse_bulk_string().map(Primitive::String)),
            '*' => Some(self.parse_array().map(Primitive::Array)),
            c => Some(Err(CommandError::UnexpectedCharacter(c))),
        }
    }
}
