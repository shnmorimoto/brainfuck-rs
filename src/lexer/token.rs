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
            Incr => write!(f, "+"),
            Decr => write!(f, "-"),
            Next => write!(f, ">"),
            Prev => write!(f, "<"),
            Read => write!(f, ","),
            Write => write!(f, "."),
            LParen => write!(f, "["),
            RParen => write!(f, "]"),
        }
    }
}

pub type Token = Annot<TokenKind>;

impl Token {
    pub fn incr(loc: Loc) -> Self {
        Self::new(TokenKind::Incr, loc)
    }

    pub fn decr(loc: Loc) -> Self {
        Self::new(TokenKind::Decr, loc)
    }

    pub fn next(loc: Loc) -> Self {
        Self::new(TokenKind::Next, loc)
    }

    pub fn prev(loc: Loc) -> Self {
        Self::new(TokenKind::Prev, loc)
    }

    pub fn read(loc: Loc) -> Self {
        Self::new(TokenKind::Read, loc)
    }

    pub fn write(loc: Loc) -> Self {
        Self::new(TokenKind::Write, loc)
    }

    pub fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }

    pub fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
}
