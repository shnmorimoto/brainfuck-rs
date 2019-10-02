use crate::lexer::token::Token;
use super::error::ParseError;
use super::ast::*;
use std::iter::Peekable;


pub fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    let ret = parse_expr(&mut tokens)?;
    match tokens.next() {
        Some(tok) => Err(ParseError::RedudantExpression(tok)),
        None => Ok(ret),
    }
}

fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    parse_expr3(tokens)
}