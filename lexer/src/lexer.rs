use crate::tokens::Tokens;

pub struct Lexer {}

impl Lexer {
    pub fn tokenize(input: &str) -> Vec<Tokens> {
        let input = input.as_bytes();
        let mut tokens = vec![];
        let mut index = 0;

        while index < input.len() {
            let token = Tokens::try_from(input[index]);

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
                let next = Tokens::try_from(input[index]);

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

    #[test]
    fn empty_input() {
        let tokens = Lexer::tokenize("     ");
        insta::assert_debug_snapshot!(tokens);
        assert!(tokens.is_empty());
    }

    #[test]
    fn single_spaced_token() {
        let tokens = Lexer::tokenize("<<       <<");
        insta::assert_debug_snapshot!(tokens);
        assert_eq!(tokens.last().unwrap(), &Tokens::MoveLeft(4));
    }

    #[test]
    fn changing_tokens() {
        let tokens = Lexer::tokenize("<<  >>");
        insta::assert_debug_snapshot!(tokens);
        assert_eq!(tokens.first().unwrap(), &Tokens::MoveLeft(2));
        assert_eq!(tokens.last().unwrap(), &Tokens::MoveRight(2));
    }

    #[test]
    fn ignoring_non_tokens() {
        let tokens = Lexer::tokenize(" _*  <<random_!?anything!!>!!>");
        insta::assert_debug_snapshot!(tokens);
        assert_eq!(tokens.first().unwrap(), &Tokens::MoveLeft(2));
    }

    #[test]
    fn pretty_hello_world() {
        let code = include_str!("../../samples/hello_world_pretty.bf");
        let tokens = Lexer::tokenize(code);
        insta::assert_debug_snapshot!(tokens);
        assert_eq!(tokens.len(), 58);
    }
}
