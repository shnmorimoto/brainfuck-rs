use common::Annot;

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