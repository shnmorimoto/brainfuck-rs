use crate::common::Annot;
use crate::common::Loc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstKind {
    Incr,
    Decr,
    Next,
    Prev,
    Read,
    Write,
    Loop(Vec<Ast>),
}

pub type Ast = Annot<AstKind>;

impl Ast {
    pub fn incr(loc: Loc) -> Self {
        Self::new(AstKind::Incr, loc)
    }

    pub fn decr(loc: Loc) -> Self {
        Self::new(AstKind::Decr, loc)
    }

    pub fn next(loc: Loc) -> Self {
        Self::new(AstKind::Next, loc)
    }

    pub fn prev(loc: Loc) -> Self {
        Self::new(AstKind::Prev, loc)
    }

    pub fn read(loc: Loc) -> Self {
        Self::new(AstKind::Read, loc)
    }

    pub fn write(loc: Loc) -> Self {
        Self::new(AstKind::Write, loc)
    }

    pub fn ast_loop(asts: Vec<Ast>, loc: Loc) -> Self {
        Self::new(AstKind::Loop(asts), loc)
    }
}
