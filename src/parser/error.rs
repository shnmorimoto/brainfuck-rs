use crate::common::Loc;
use crate::lexer::token::Token;
use std::fmt;
use std::error::Error as StdError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    UnexpectedToken(Token),
    NotExpression(Token),
    UnclosedOpenParen(Loc),
    RedudantClosedParen(Token),
    RedudantExpression(Token),
    Eof,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;
        match self {
            UnexpectedToken(tok) => write!(f, "{}: {} is not expected", tok.loc, tok.value),
            NotExpression(tok) => write!(
                f,
                "{}: '{}' is not a start of expression",
                tok.loc, tok.value
            ),
            UnclosedOpenParen(position) => write!(f, "{}: Paren is not closed", position),
            RedudantClosedParen(tok) => {
                write!(f, "{}: '{}' is redudant closed", tok.loc, tok.value)
            }
            RedudantExpression(tok) => write!(
                f,
                "{}: expression after '{}' is redundant",
                tok.loc, tok.value
            ),
            Eof => write!(f, "End of file"),
        }
    }
}

impl StdError for ParseError {}