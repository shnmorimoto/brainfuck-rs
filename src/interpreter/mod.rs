use crate::parser::ast::Ast;
use crate::parser::ast::AstKind;
use error::InterpreterError;
use std::io;

mod error;

pub struct Interpreter {
    position: usize,
    tape: Vec<u32>,
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

        fn _eval(_position: &mut usize,  _tape: &mut Vec<u32>, _size: usize, _expr: &Vec<Ast>) -> Result<(), InterpreterError> {
            for ast in _expr.into_iter() {
                match &ast.value {
                    AstKind::Incr => _tape[*_position] += 1,
                    AstKind::Decr => _tape[*_position] -= 1,
                    AstKind::Next => { 
                        if *_position + 1 >= _size - 1 {
                            return Err(InterpreterError::buffer_overflow(ast.loc))
                        } else {
                            *_position += 1
                        }
                    }
                    AstKind::Prev => {
                        if *_position == 0 {
                            return Err(InterpreterError::negative_postion(ast.loc))
                        } else {
                            *_position -= 1
                        }
                    }
                    AstKind::Write => {
                        let decoded_char = std::char::from_u32(_tape[*_position]);
                        match decoded_char {
                            Some(c) => print!("{}", c),
                            _ => return Err(InterpreterError::cannot_decode_character(ast.loc))
                        }
                    }
                    AstKind::Read => {
                        let mut buf_in = String::new();
                        io::stdin().read_line(&mut buf_in);
                        _tape[*_position] = match buf_in.trim().parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => {
                                buf_in.chars().collect::<Vec<char>>()[0] as u32
                            }
                        };
                    }
                    AstKind::Loop(inner_ast) => {
                        while _tape[*_position] != 0 {
                            _eval(_position, _tape, _size, &inner_ast)?
                        }
                    }
                }
            }
            Ok(())
        }
        _eval(&mut self.position, &mut self.tape, self.size, &expr)
    }
}