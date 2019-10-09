use crate::common::Annot;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum InterpreterErrorKind {
    TapeBufferOverflow,
}

type InterpreterError = Annot<InterpreterErrorKind>;