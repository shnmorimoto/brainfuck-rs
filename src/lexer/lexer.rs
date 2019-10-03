use super::error::LexError;
use super::token::Token;
use crate::common::Loc;

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
            b'+' => lex_a_token!(lex_incr(input, pos)),
            b'-' => lex_a_token!(lex_decr(input, pos)),
            b'>' => lex_a_token!(lex_next(input, pos)),
            b'<' => lex_a_token!(lex_prev(input, pos)),
            b',' => lex_a_token!(lex_read(input, pos)),
            b'.' => lex_a_token!(lex_write(input, pos)),
            b'[' => lex_a_token!(lex_lparen(input, pos)),
            b']' => lex_a_token!(lex_rparen(input, pos)),
            b' ' | b'\n' | b'\t' => {
                let ((), p) = skip_spaces(input, pos)?;
                pos = p;
            }
            b => return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1))),
        }
    }
    Ok(tokens)
}

fn consume_byte(input: &[u8], pos: usize, b: u8) -> Result<(u8, usize), LexError> {
    if input.len() <= pos {
        return Err(LexError::eof(Loc(pos, pos)));
    }

    if input[pos] != b {
        return Err(LexError::invalid_char(
            input[pos] as char,
            Loc(pos, pos + 1),
        ));
    }

    Ok((b, pos + 1))
}

fn lex_incr(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'+').map(|(_, end)| (Token::incr(Loc(start, end)), end))
}

fn lex_decr(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'-').map(|(_, end)| (Token::decr(Loc(start, end)), end))
}

fn lex_next(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'>').map(|(_, end)| (Token::next(Loc(start, end)), end))
}

fn lex_prev(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'<').map(|(_, end)| (Token::prev(Loc(start, end)), end))
}

fn lex_read(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b',').map(|(_, end)| (Token::read(Loc(start, end)), end))
}

fn lex_write(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'.').map(|(_, end)| (Token::write(Loc(start, end)), end))
}

fn lex_lparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'[').map(|(_, end)| (Token::lparen(Loc(start, end)), end))
}

fn lex_rparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b']').map(|(_, end)| (Token::rparen(Loc(start, end)), end))
}

fn skip_spaces(input: &[u8], pos: usize) -> Result<((), usize), LexError> {
    let pos = recognize_many(input, pos, |b| b" \n\t".contains(&b));
    Ok(((), pos))
}

fn recognize_many(input: &[u8], mut pos: usize, mut f: impl FnMut(u8) -> bool) -> usize {
    while pos < input.len() && f(input[pos]) {
        pos += 1;
    }
    pos
}

#[test]
fn test_lexer() {
    assert_eq!(
        lex("+-><,.[] +"),
        Ok(vec![
            Token::incr(Loc(0, 1)),
            Token::decr(Loc(1, 2)),
            Token::next(Loc(2, 3)),
            Token::prev(Loc(3, 4)),
            Token::read(Loc(4, 5)),
            Token::write(Loc(5, 6)),
            Token::lparen(Loc(6, 7)),
            Token::rparen(Loc(7, 8)),
            Token::incr(Loc(9, 10)),
        ])
    )
}
