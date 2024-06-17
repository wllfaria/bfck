#[derive(Debug, Clone, Copy)]
pub enum Tokens {
    MoveLeft(usize),
    MoveRight(usize),
    Increment(usize),
    Decrement(usize),
    Write(usize),
    Read(usize),
    JumpIfZero(usize),
    JumpUnlessZero(usize),
}

impl PartialEq for Tokens {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Tokens::MoveLeft(_), Tokens::MoveLeft(_))
                | (Tokens::MoveRight(_), Tokens::MoveRight(_))
                | (Tokens::Increment(_), Tokens::Increment(_))
                | (Tokens::Decrement(_), Tokens::Decrement(_))
                | (Tokens::Write(_), Tokens::Write(_))
                | (Tokens::Read(_), Tokens::Read(_))
                | (Tokens::JumpIfZero(_), Tokens::JumpIfZero(_))
                | (Tokens::JumpUnlessZero(_), Tokens::JumpUnlessZero(_))
        )
    }
}

impl Tokens {
    pub fn inner(&mut self) -> &mut usize {
        match self {
            Tokens::MoveLeft(count) => count,
            Tokens::MoveRight(count) => count,
            Tokens::Increment(count) => count,
            Tokens::Decrement(count) => count,
            Tokens::Write(count) => count,
            Tokens::Read(count) => count,
            Tokens::JumpIfZero(count) => count,
            Tokens::JumpUnlessZero(count) => count,
        }
    }
}

impl TryFrom<u8> for Tokens {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Tokens, Self::Error> {
        match value {
            b'<' => Ok(Tokens::MoveLeft(1)),
            b'>' => Ok(Tokens::MoveRight(1)),
            b'+' => Ok(Tokens::Increment(1)),
            b'-' => Ok(Tokens::Decrement(1)),
            b'.' => Ok(Tokens::Write(1)),
            b',' => Ok(Tokens::Read(1)),
            b'[' => Ok(Tokens::JumpIfZero(1)),
            b']' => Ok(Tokens::JumpUnlessZero(1)),
            _ => Err("not a valid token"),
        }
    }
}
