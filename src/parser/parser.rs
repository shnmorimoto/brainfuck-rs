use crate::lexer::token::Token;
use super::error::ParseError;
use super::ast::*;


pub fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    let ret = parse_expr(&mut tokens)?;
    match tokens.next() {
        Some(tok) => Err(ParseError::RedudantExpression(tok)),
        None => Ok(ret),
    }
}