use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);

impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Annot<T> {
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self {value, loc}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TokenKind {
    Incr,
    Decr,
    Next,
    Prev,
    Read,
    Write,
    Open,
    Close,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TokenKind::*;
        match self {
            Incr => write!(f, ">")
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