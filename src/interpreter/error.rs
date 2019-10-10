use crate::common::Annot;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterpreterErrorKind {
    TapeBufferOverflow,
}

pub type InterpreterError = Annot<InterpreterErrorKind>;