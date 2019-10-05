use super::ast::*;
use super::error::ParseError;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use std::iter::Peekable;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Ast>, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    let ret = parse_expr(&mut tokens)?;
    match tokens.next() {
        Some(tok) => Err(ParseError::RedudantExpression(tok)),
        None => Ok(ret),
    }
}

fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Vec<Ast>, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut instruction_stack:Vec<Ast> = Vec::new();
    loop {
        let ast = tokens
            .next()
            .ok_or(ParseError::Eof)
            .and_then(|tok| match tok.value {
                TokenKind::Incr => Ok(Ast::incr(tok.loc)),
                TokenKind::Decr => Ok(Ast::decr(tok.loc)),
                TokenKind::Next => Ok(Ast::next(tok.loc)),
                TokenKind::Prev => Ok(Ast::prev(tok.loc)),
                TokenKind::Read => Ok(Ast::read(tok.loc)),
                TokenKind::Write => Ok(Ast::write(tok.loc)),
            });

        match ast {
            Err(ParseError::Eof) => break,
            _ => (),
        }
        instruction_stack.push(ast?);
    }
    Ok(instruction_stack)
}