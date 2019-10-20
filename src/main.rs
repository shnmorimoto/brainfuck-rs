use brainfuck_rs::interpreter::Interpreter;
use brainfuck_rs::parser::parser::parse;
use brainfuck_rs::lexer::lexer::lex;
use brainfuck_rs::parser::ast::Ast;
use brainfuck_rs::error::Error;
use brainfuck_rs::error::show_trace;
use std::env;
use std::fs::read_to_string;
use std::process;

fn main() {
    let mut interp = Interpreter::new(3000);

    let filename = env::args().nth(1).expect("1 argument FILENAME required");
    let code = match read_to_string(filename) {
        Ok(code) => code,
        Err(e) => {
            println!("Can't open file: {}", e);
            process::exit(-1);
        }
    };


    let asts = match parse_code(&code) {
        Ok(asts) => asts,
        Err(e) => {
            e.show_diagnostic(&code);
            show_trace(e);
            process::exit(-1);
        }
    };

    match interp.eval(asts) {
        Ok(_) => return (),
        Err(e) => {
            e.show_diagnostic(&code);
            show_trace(e);
            process::exit(-1)
        }
    }
}

fn parse_code(code: &str) -> Result<Vec<Ast>, Error> {
    let tokens = match lex(code) {
        Ok(tokens) => tokens,
        Err(e) => return Err(Error::Lexer(e)),
    };
    let asts = match parse(tokens) {
        Ok(asts) => asts,
        Err(e) => return Err(Error::Parser(e)),
    };
    Ok(asts)
}