use std::error::Error as StdError;
use crate::lexer::error::LexError;
use crate::parser::error::ParseError;
use crate::common::Loc;
use crate::lexer::token::Token;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    Lexer(LexError),
    Parser(ParseError),
}

impl Error {
    pub fn show_diagnostic(&self, input: &str) {
        use self::Error::*;
        use self::ParseError as P;
        let (e, loc): (&dyn StdError, Loc) = match self {
            Lexer(e) => (e, e.loc.clone()),
            Parser(e) => {
                let loc = match e {
                    P::UnexpectedToken(Token {loc, ..})
                    | P::NotExpression(Token {loc, ..})
                    | P::UnclosedOpenParen(loc)
                    | P::RedudantClosedParen(Token {loc, ..}) => loc.clone(),
                    P::RedudantExpression(Token {loc, ..}) => Loc(loc.0, input.len()),
                    P::Eof => Loc(input.len(), input.len() + 1),
                };
                (e, loc)
            }
        };
        eprintln!("{}", e);
        print_annot(input, loc);
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parser error")
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use self::Error::*;
        match self {
            Lexer(lex) => Some(lex),
            Parser(parse) => Some(parse),
        }
    }
}

fn print_annot(input: &str, loc: Loc) {
    eprintln!("{}", input);
    eprintln!("{}{}", " ".repeat(loc.0), "^".repeat(loc.1 - loc.0));
}

pub fn show_trace<E: StdError>(e: E) {
    eprintln!("{}", e);
    let mut source = e.source();
    while let Some(e) = source {
        eprintln!("caused by {}", e);
        source = e.source()
    }
}