use crate::common::{Annot, Loc};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Incr,
    Decr,
    Next,
    Prev,
    Read,
    Write,
    LParen,
    RParen,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TokenKind::*;
        match self {
            Incr => write!(f, ">"),
    
            Decr => write!(f, "<"),
            Next => write!(f, "+"),
            Prev => write!(f, "-"),
            Read => write!(f, ","),
            Write => write!(f, "."),
            LParen => write!(f, "["),
            RParen => write!(f, "]"),
        }
    }
}

type Token = Annot<TokenKind>;

impl Token {
    fn incr(loc: Loc) -> Self {
        Self::new(TokenKind::Incr, loc)
    }

    fn decr(loc: Loc) -> Self {
        Self::new(TokenKind::Decr, loc)
    }

    fn next(loc: Loc) -> Self {
        Self::new(TokenKind::Next, loc)
    }

    fn prev(loc: Loc) -> Self {
        Self::new(TokenKind::Prev, loc)
    }

    fn read(loc: Loc) -> Self {
        Self::new(TokenKind::Read, loc)
    }

    fn write(loc: Loc) -> Self {
        Self::new(TokenKind::Write, loc)
    }

    fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }

    fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
}
