use super::ast::*;
use super::error::ParseError;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
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
    parse_atom(tokens)
}

fn parse_atom<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    tokens
        .next()
        .ok_or(ParseError::Eof)
        .and_then(|tok| match tok.value {
            TokenKind::Incr => Ok(Ast::incr(tok.loc)),
            TokenKind::Decr => Ok(Ast::decr(tok.loc)),
            TokenKind::Next => Ok(Ast::next(tok.loc)),
            TokenKind::Prev => Ok(Ast::prev(tok.loc)),
            TokenKind::Read => Ok(Ast::read(tok.loc)),
            TokenKind::Write => Ok(Ast::write(tok.loc)),
            TokenKind::LParen => {
                let e = parse_expr(tokens)?;
                match tokens.next() {
                    Some(Token {
                        value: TokenKind::RParen,
                        ..
                    }) => Ok(e),
                    Some(t) => Err(ParseError::RedudantExpression(t)),
                    _ => Err(ParseError::UnclosedOpenParen(tok)),
                }
            }
            _ => Err(ParseError::NotExpression(tok)),
        })
}
