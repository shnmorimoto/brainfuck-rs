use brainfuck_rs::interpreter::Interpreter;
use brainfuck_rs::parser::parser::parse;
use brainfuck_rs::lexer::lexer::lex;
use brainfuck_rs::parser::ast::Ast;
use brainfuck_rs::error::Error;
use brainfuck_rs::error::show_trace;

fn main() {
    let mut interp = Interpreter::new(3000);
    let code: &str = "+++++++++[>++++++++>+++++++++++>+++>+<<<<-]>.>++.+++++++..+++.>+++++.<<+++++++++++++++.>.+++.------.--------.>+.>+."; //hello world
    let asts = match parse_code(code) {
        Ok(asts) => asts,
        Err(e) => {
            e.show_diagnostic(&code);
            show_trace(e);
            return ()
        }
    };

    match interp.eval(asts) {
        Ok(_) => return (),
        Err(e) => {
            e.show_diagnostic(&code);
            show_trace(e);
            return();

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