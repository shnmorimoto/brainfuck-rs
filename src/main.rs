use brainfuck_rs::error::show_trace;
use brainfuck_rs::error::Error;
use brainfuck_rs::interpreter::Interpreter;
use brainfuck_rs::lexer::lexer::lex;
use brainfuck_rs::parser::ast::Ast;
use brainfuck_rs::parser::parser::parse;
use std::fs::read_to_string;
use std::process;
extern crate clap;
use clap::{App, Arg};

fn main() {
    const DEFAULT_TAPE_SIZE: usize = 30000;

    let opts = App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("FILENAME")
                .short("f")
                .long("file_name")
                .help("brainfuck file name")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("TAPE_SIZE")
                .short("s")
                .long("tape_size")
                .help("tape size")
                .takes_value(true),
        );

    let matches = opts.get_matches();

    let filename = match matches.value_of("FILENAME") {
        Some(filename) => filename,
        _ => {
            println!("FILENAME required");
            process::exit(-1);
        }
    };

    let tape_size = match matches.value_of("TAPE_SIZE") {
        Some(tape_size_str) => match tape_size_str.trim().parse::<usize>() {
            Ok(tape_size) => tape_size,
            _ => {
                println!(
                    "[Warn] parse error: {}. tape_size is set to default size: {}",
                    tape_size_str, DEFAULT_TAPE_SIZE
                );
                DEFAULT_TAPE_SIZE
            }
        },
        _ => DEFAULT_TAPE_SIZE,
    };

    let mut interp = Interpreter::new(tape_size);

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
