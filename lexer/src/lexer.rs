use crate::tokens::Tokens;

pub struct Lexer {}

impl Lexer {
    pub fn parse(input: &str) -> Vec<Tokens> {
        let mut tokens = vec![];
        if input.is_empty() {
            return tokens;
        }

        let input = input.as_bytes();
        let mut position = 0;

        while Tokens::try_from(input[position]).is_err() && position < input.len() - 1 {
            position += 1;
        }

        if position >= input.len() {
            return tokens;
        }

        let first_token = match Tokens::try_from(input[position]) {
            Ok(token) => token,
            Err(_) => return tokens,
        };
        position += 1;

        if position == input.len() - 1 {
            tokens.push(first_token);
            return tokens;
        }

        Lexer::reduce_tokens(input, position, first_token, &mut tokens);

        tokens
    }

    fn reduce_tokens(input: &[u8], position: usize, curr_token: Tokens, tokens: &mut Vec<Tokens>) {
        if position >= input.len() && curr_token.count() > 0 {
            tokens.push(curr_token);
            return;
        }
        let next_char = input[position];
        match (next_char, curr_token) {
            (b'<', Tokens::MoveLeft(_)) => Lexer::reduce_tokens(
                input,
                position + 1,
                Tokens::MoveLeft(curr_token.count() + 1),
                tokens,
            ),
            (b'>', Tokens::MoveRight(_)) => {
                Lexer::reduce_tokens(
                    input,
                    position + 1,
                    Tokens::MoveRight(curr_token.count() + 1),
                    tokens,
                );
            }
            (b'+', Tokens::Increment(_)) => {
                Lexer::reduce_tokens(
                    input,
                    position + 1,
                    Tokens::Increment(curr_token.count() + 1),
                    tokens,
                );
            }
            (b'-', Tokens::Decrement(_)) => {
                Lexer::reduce_tokens(
                    input,
                    position + 1,
                    Tokens::Decrement(curr_token.count() + 1),
                    tokens,
                );
            }
            (b'.', Tokens::Write(_)) => {
                Lexer::reduce_tokens(
                    input,
                    position + 1,
                    Tokens::Write(curr_token.count() + 1),
                    tokens,
                );
            }
            (b',', Tokens::Read(_)) => {
                Lexer::reduce_tokens(
                    input,
                    position + 1,
                    Tokens::Read(curr_token.count() + 1),
                    tokens,
                );
            }
            (b'[', Tokens::JumpIfZero(_)) => {
                Lexer::reduce_tokens(
                    input,
                    position + 1,
                    Tokens::JumpIfZero(curr_token.count() + 1),
                    tokens,
                );
            }
            (b']', Tokens::JumpUnlessZero(_)) => {
                Lexer::reduce_tokens(
                    input,
                    position + 1,
                    Tokens::JumpUnlessZero(curr_token.count() + 1),
                    tokens,
                );
            }
            (b'<', _) => {
                tokens.push(curr_token);
                Lexer::reduce_tokens(input, position + 1, Tokens::MoveLeft(1), tokens);
            }
            (b'>', _) => {
                tokens.push(curr_token);
                Lexer::reduce_tokens(input, position + 1, Tokens::MoveRight(1), tokens);
            }
            (b'+', _) => {
                tokens.push(curr_token);
                Lexer::reduce_tokens(input, position + 1, Tokens::Increment(1), tokens);
            }
            (b'-', _) => {
                tokens.push(curr_token);
                Lexer::reduce_tokens(input, position + 1, Tokens::Decrement(1), tokens);
            }
            (b'.', _) => {
                tokens.push(curr_token);
                Lexer::reduce_tokens(input, position + 1, Tokens::Write(1), tokens);
            }
            (b',', _) => {
                tokens.push(curr_token);
                Lexer::reduce_tokens(input, position + 1, Tokens::Read(1), tokens);
            }
            (b'[', _) => {
                tokens.push(curr_token);
                Lexer::reduce_tokens(input, position + 1, Tokens::JumpIfZero(1), tokens);
            }
            (b']', _) => {
                tokens.push(curr_token);
                Lexer::reduce_tokens(input, position + 1, Tokens::JumpUnlessZero(1), tokens);
            }
            _ => Lexer::reduce_tokens(input, position + 1, curr_token, tokens),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        let tokens = Lexer::parse("     ");
        insta::assert_debug_snapshot!(tokens);
        assert!(tokens.is_empty());
    }

    #[test]
    fn single_spaced_token() {
        let tokens = Lexer::parse("<<       <<");
        insta::assert_debug_snapshot!(tokens);
        assert_eq!(tokens.last().unwrap(), &Tokens::MoveLeft(4));
    }

    #[test]
    fn changing_tokens() {
        let tokens = Lexer::parse("<<  >>");
        insta::assert_debug_snapshot!(tokens);
        assert_eq!(tokens.first().unwrap(), &Tokens::MoveLeft(2));
        assert_eq!(tokens.last().unwrap(), &Tokens::MoveRight(2));
    }

    #[test]
    fn ignoring_non_tokens() {
        let tokens = Lexer::parse(" _*  <<random_!?anything!!>!!>");
        insta::assert_debug_snapshot!(tokens);
        assert_eq!(tokens.first().unwrap(), &Tokens::MoveLeft(2));
        assert_eq!(tokens.last().unwrap(), &Tokens::MoveRight(2));
    }
}
