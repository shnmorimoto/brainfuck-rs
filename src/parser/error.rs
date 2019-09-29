use crate::lexer::token::Token;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    UnexpectedToken(Token),
    NotExpression(Token),
    UnclosedOpenParen(Token),
    Eof,
}