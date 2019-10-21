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
                    if *stack_num == 0 {
                        Err(ParseError::RedudantClosedParen(tok.clone()))
                    } else {
                        *stack_num -= 1;
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

#[test]
fn test_parse() {
    let ast = parse(vec![
        Token::incr(Loc(0, 1)),
        Token::decr(Loc(1, 2)),
        Token::next(Loc(2, 3)),
        Token::prev(Loc(3, 4)),
        Token::read(Loc(4, 5)),
        Token::write(Loc(5, 6)),
        Token::lparen(Loc(6, 7)),
        Token::incr(Loc(7, 8)),
        Token::decr(Loc(8, 9)),
        Token::rparen(Loc(9, 10)),
        Token::incr(Loc(10, 11)),
    ]);
    assert_eq!(
        ast,
        Ok(vec![
            Ast::incr(Loc(0, 1)),
            Ast::decr(Loc(1, 2)),
            Ast::next(Loc(2, 3)),
            Ast::prev(Loc(3, 4)),
            Ast::read(Loc(4, 5)),
            Ast::write(Loc(5, 6)),
            Ast::ast_loop(vec![Ast::incr(Loc(7, 8)), Ast::decr(Loc(8, 9)),], Loc(6, 7)),
            Ast::incr(Loc(10, 11))
        ])
    )
}
