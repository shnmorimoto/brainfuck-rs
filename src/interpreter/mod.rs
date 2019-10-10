use crate::parser::ast::Ast;
use crate::parser::ast::AstKind;
use error::InterpreterError;

mod error;

struct Interpreter;

impl Interpreter {

    pub fn new() -> Self {
        Interpreter
    }

    pub fn eval(&mut self, expr: Vec<Ast>) -> () {
        ()
    }
}