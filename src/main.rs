use brainfuck_rs::interpreter::Interpreter;
use brainfuck_rs::parser::parser::parse;
use brainfuck_rs::lexer::lexer::lex;

fn main() {
    let mut interp = Interpreter::new(3000);
    let code: &str = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.>++++++++++."; //ABC
    let tokens = match lex(code) {
        Ok(tokens) => tokens,
        Err(_) => return (),
    };
    let asts = match parse(tokens) {
        Ok(asts) => asts,
        Err(_) => return (),
    };
    match interp.eval(asts) {
        Ok(_) => return (),
        Err(_) => return (),
    }
}