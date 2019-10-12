use crate::parser::ast::Ast;
use crate::parser::ast::AstKind;
use error::InterpreterError;

mod error;

struct Interpreter {
    position: usize,
    tape: Vec<u64>,
    size: usize,
}

impl Interpreter {

    pub fn new(tape_size: usize) -> Self {
        Interpreter {
            position: 0,
            size: tape_size,
            tape: vec![0; tape_size],
        }
    }

    pub fn eval(&mut self, expr: Vec<Ast>) -> Result<(), InterpreterError> {
        for ast in expr.into_iter() {
            match ast.value {
                AstKind::Incr => self.tape[self.position] += 1,
                AstKind::Decr => self.tape[self.position] -= 1,
                AstKind::Next => { 
                    if self.position + 1 >= self.size - 1 {
                        return Err(InterpreterError::buffer_overflow(ast.loc))
                    } else {
                        self.position += 1
                    }
                }
                AstKind::Prev => {
                    if self.position - 1 < 0 {
                        return Err(InterpreterError::negative_postion(ast.loc))
                    } else {
                        self.position -= 1
                    }
                }
                _ => ()
            }
        }
        Ok(())
    }
}