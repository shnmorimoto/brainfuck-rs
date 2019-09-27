use super::error::LexError;
use super::token::Token;

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {

    let mut tokens = Vec::new();
    let input = input.as_bytes();
    let mut pos = 0;

    macro_rules! lex_a_token {
        ($lexer:expr) => {{
            let (tok, p) = $lexer?;
            tokens.push(tok);
            pos = p;
        }};
    }

    while pos < input.len() {
        match input[pos] {
            b'0' ..= b'9' => lex_a_token!((input, pos)),
            b'+' => lex_a_token!(lex_plus(input, pos)),
            b'-' => lex_a_token!(lex_minus(input, pos)),
            b'*' => lex_a_token!(lex_asterisk(input, pos)),
            b'/' => lex_a_token!(lex_slash(input, pos)),
            b'(' => lex_a_token!(lex_lparen(input, pos)),
            b')' => lex_a_token!(lex_rparen(input, pos)),
            b' ' | b'\n' | b'\t' => {
                let ((), p) = skip_spaces(input, pos)?;
                pos = p;
            },
            b => return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1))),
        }
    }
    Ok(tokens)
}