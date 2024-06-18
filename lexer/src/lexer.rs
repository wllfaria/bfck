use crate::token::Token;

/// Lexer for the brainfuck source code, exposes only one method, namely `tokenize`.
/// The responsibility of the lexer is to receive a code string and tokenize however
/// many tokens are available into semantic tokens.
///
/// This lexer also reduces the tokens to a intermediate representation, where
/// instead of having a `Token` for possibly every byte in the source code, we reduce
/// it to an intermediate representation that collapses repeating tokens into a single
/// one.
///
/// example:
/// given the brainfuck source code `>>>-----..`; the lexer would produce a vector like
/// the following
/// ```rust
/// use lexer::Lexer;
/// use lexer::token::Token;
///
/// let source = ">>>-----..";
/// let expected = vec![
///     Token::MoveRight(3),
///     Token::Decrement(5),
///     Token::Write(2),
/// ];
///
/// let tokens = Lexer::tokenize(source);
/// assert_eq!(tokens, expected);
/// ```
pub struct Lexer {}

impl Lexer {
    /// given a input slice of source code, Lexer will try to tokenize it ignoring all
    /// whitespaces and non-tokens and always give back a vector of tokens, which
    /// could be empty if no valid tokens exists within the input string
    ///
    /// no validations are performed here purposefully, its not the responsibility of
    /// the lexer to validate if the source code is valid.
    pub fn tokenize(input: &str) -> Vec<Token> {
        let input = input.as_bytes();
        let mut tokens = vec![];
        let mut index = 0;

        while index < input.len() {
            let token = Token::try_from(input[index]);

            // we couldn't convert the byte to a valid token, so we just consider it
            // a comment and skip over it
            if token.is_err() {
                index += 1;
                continue;
            }

            // its obviously safe to unwrap here since we check above, but lets make
            // it clear this is intended!
            assert!(token.is_ok(), "token was not valid yet we didn't skip it");
            let mut token = token.unwrap();

            // since we found a valid token, we will keep collecting tokens until we
            // find a different one
            index += 1;
            while index < input.len() {
                let next = Token::try_from(input[index]);

                // again, if the byte is a non-valid brainfuck token, we just skip
                // over it
                if next.is_err() {
                    index += 1;
                    continue;
                }

                // if it is a valid token, but its a different one from the one we
                // initially collected, we break out of this loop without
                // incrementing the index, so the next outer loop iteration will
                // resume from there
                //
                // its obviously safe to unwrap here since we check above, but lets make
                // it clear this is intended!
                assert!(next.is_ok(), "token was not valid yet we didn't skip it");
                if next.unwrap().ne(&token) {
                    break;
                }

                // if the tokens are the same, we increase the count of the token
                // through the inner() helper and increase the index to we do the
                // same with the next
                *token.inner() += 1;
                index += 1;
            }

            tokens.push(token);
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &'static str) -> Vec<Token> {
        Lexer::tokenize(input)
    }

    #[test]
    fn empty_input() {
        insta::assert_debug_snapshot!(check("     "));
    }

    #[test]
    fn single_spaced_token() {
        insta::assert_debug_snapshot!(check("<<       <<"));
    }

    #[test]
    fn changing_tokens() {
        insta::assert_debug_snapshot!(check("<<  >>"));
    }

    #[test]
    fn ignoring_non_tokens() {
        insta::assert_debug_snapshot!(check(" _*  <<random_!?anything!!>!!>"));
    }

    #[test]
    fn pretty_hello_world() {
        insta::assert_debug_snapshot!(check(include_str!("../../samples/hello_world_pretty.bf")));
    }
}
