/// brainfuck has only 8 commands, those are all described below, you can refer
/// to each of them in more detail following the link below
///
/// https://en.wikipedia.org/wiki/Brainfuck
#[derive(Debug, Clone, Copy)]
pub enum Token {
    /// move (decrement) the data pointer by one, essentially pointing to the
    /// next cell to the left
    MoveLeft(usize),
    /// move (increment) the data pointer by one, essentially pointing to the
    /// next cell to the right
    MoveRight(usize),
    /// increment the byte at the data pointer by one, as the specification
    /// mentions, this wraps when `u8::MAX` is reached, going back to 0
    Increment(usize),
    /// decrement the byte at the data pointer by one, as the specification
    /// mentions, this wraps when `0` is reached, going back to `u8::MAX`
    Decrement(usize),
    /// outputs the byte at the data pointer
    Write(usize),
    /// reads one byte from stdin
    Read(usize),
    /// if the byte at the data pointer is zero, then instead of moving the
    /// instruction pointer to the next instruction, move it to the instruction
    /// right after the matching closing ´]´
    JumpIfZero(usize),
    /// if the byte at the data pointer is non zero, then instead of moving the
    /// instruction pointer to the next instruction, move it back to the
    /// instruction right after the matching opening ´[´
    JumpUnlessZero(usize),
}

/// this custom partial equality check exists as for this implementation, we
/// don't care about the inner value, which is the count of times this token
/// was repeated. We only care if both tokens are the same, so we can match
/// any new token which will always have count 1, to an existing token and
/// as long as they are the same type, we can increment the ineer count.
///
/// this is how we reduce the tokens to a smaller amount by grouping by
/// repetition
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Token::MoveLeft(_), Token::MoveLeft(_))
                | (Token::MoveRight(_), Token::MoveRight(_))
                | (Token::Increment(_), Token::Increment(_))
                | (Token::Decrement(_), Token::Decrement(_))
                | (Token::Write(_), Token::Write(_))
                | (Token::Read(_), Token::Read(_))
                | (Token::JumpIfZero(_), Token::JumpIfZero(_))
                | (Token::JumpUnlessZero(_), Token::JumpUnlessZero(_))
        )
    }
}

impl Token {
    /// helper function to get the inner count as a mutable reference, this is
    /// used to increment the count whenever we found a repeated tokens in a
    /// sequence
    pub fn inner(&mut self) -> &mut usize {
        match self {
            Token::MoveLeft(count) => count,
            Token::MoveRight(count) => count,
            Token::Increment(count) => count,
            Token::Decrement(count) => count,
            Token::Write(count) => count,
            Token::Read(count) => count,
            Token::JumpIfZero(count) => count,
            Token::JumpUnlessZero(count) => count,
        }
    }
}

/// when tokenizing the input, we convert the string slice into a byte array,
/// which is easier to handle while keeping the tokenizer 0 copy. this is a
/// convenience impl that makes it easier to find out wether we should skip
/// the token when its not any of the valid tokens, or collect it into a
/// existing, or new group of tokens
impl TryFrom<u8> for Token {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Token, Self::Error> {
        match value {
            b'<' => Ok(Token::MoveLeft(1)),
            b'>' => Ok(Token::MoveRight(1)),
            b'+' => Ok(Token::Increment(1)),
            b'-' => Ok(Token::Decrement(1)),
            b'.' => Ok(Token::Write(1)),
            b',' => Ok(Token::Read(1)),
            b'[' => Ok(Token::JumpIfZero(1)),
            b']' => Ok(Token::JumpUnlessZero(1)),
            _ => Err("not a valid token"),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::MoveLeft(count) => write!(f, "{}<", count),
            Token::MoveRight(count) => write!(f, "{}>", count),
            Token::Increment(count) => write!(f, "{}-", count),
            Token::Decrement(count) => write!(f, "{}+", count),
            Token::Write(count) => write!(f, "{}.", count),
            Token::Read(count) => write!(f, "{},", count),
            Token::JumpIfZero(count) => write!(f, "{}[", count),
            Token::JumpUnlessZero(count) => write!(f, "{}]", count),
        }
    }
}
