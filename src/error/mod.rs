use std::error::Error as StdError;
use crate::lexer::error::LexError;
use crate::parser::error::ParseError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    Lexer(LexError),
    Parser(ParseError),
}