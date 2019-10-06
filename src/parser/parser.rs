use super::ast::*;
use super::error::ParseError;
use crate::common::Loc;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use std::iter::Peekable;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Ast>, ParseError> {
    let last_pos = Loc(tokens.len(), tokens.len() + 1);
    let mut tokens = tokens.into_iter().peekable();
    let mut stack_num: u32 = 0;
    let (ret, stack_num) = parse_expr(&mut tokens, &mut stack_num)?;

    if stack_num != 0 {
        return Err(ParseError::UnclosedOpenParen(last_pos));
    }

    match tokens.next() {
        Some(tok) => Err(ParseError::RedudantExpression(tok)),
        None => Ok(ret),
    }
}

fn parse_expr<Tokens>(
    tokens: &mut Peekable<Tokens>,
    stack_num: &mut u32,
) -> Result<(Vec<Ast>, u32), ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut instruction_stack: Vec<Ast> = Vec::new();
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
                TokenKind::LParen => {
                    *stack_num += 1;
                    let (loop_asts, _) = parse_expr(tokens, stack_num)?;
                    Ok(Ast::ast_loop(loop_asts, tok.loc))
                }
                TokenKind::RParen => {
                    *stack_num -= 1;
                    if *stack_num < 0 {
                        Err(ParseError::RedudantClosedParen(tok.clone()))
                    } else {
                        Err(ParseError::Eof)
                    }
                }
            });
        match ast {
            Err(ParseError::Eof) => break,
            _ => (),
        }
        instruction_stack.push(ast?);
    }
    Ok((instruction_stack, *stack_num))
}
