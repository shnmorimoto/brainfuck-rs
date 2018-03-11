use std::env;
use std::process;
use std::fs;
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::error::Error;
use std::collections::HashMap;

const MAX_BUFFER_SIZE: usize = 30000;


enum BFConstruct {
    Incr,
    Decr,
    Next,
    Prev,
    Read,
    Write,
    Open,
    Close,
}

fn main() {

    let mut constructs = HashMap::new();

    constructs.insert('+', BFConstruct::Incr);
    constructs.insert('-', BFConstruct::Decr);
    constructs.insert('>', BFConstruct::Next);
    constructs.insert('<', BFConstruct::Prev);
    constructs.insert(',', BFConstruct::Read);
    constructs.insert('.', BFConstruct::Write);
    constructs.insert('[', BFConstruct::Open);
    constructs.insert(']', BFConstruct::Close);

    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} BF_FILE", args[0]);
        process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let display = file_path.display();
    let mut f = match fs::File::open(file_path) {
        Ok(file) => BufReader::new(file),
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why))
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Err(why) => panic!("{} contains:\n{}", display, Error::description(&why)),
        Ok(_) => println!("BF Source: {}", &s)
    }

    let src = s.chars().collect::<Vec<char>>();

    let mut b = vec![0; MAX_BUFFER_SIZE];
    let mut i = 0;
    let mut si = 0;
    let mut n = 0;
    let mut buf: Vec<u32> = vec![0; 1];

    loop {

        match constructs.get(&src[si]) {
            Some(&BFConstruct::Incr) => b[i] = (b[i] + 1) % 256,
            Some(&BFConstruct::Decr) => {
                b[i] = if (b[i] as i32 - 1) < 0 {
                    255
                } else {
                    b[i] - 1
                }
            },
            Some(&BFConstruct::Next) => i = i + 1,
            Some(&BFConstruct::Prev) => i = i - 1,
            Some(&BFConstruct::Write) => {
                buf[0] = b[i];
                print!("{}", std::char::from_u32(buf[0]).unwrap());
            },
            Some(&BFConstruct::Read) => {
                let mut buf_s = String::new();
                io::stdin().read_line(&mut buf_s);
                b[i] = match buf_s.trim().parse::<u32>() {
                    Ok(num) => num,
                    Err(e) => buf_s.chars().collect::<Vec<char>>()[0] as u32
                };
            },
            Some(&BFConstruct::Open) => {
                if b[i] == 0 {
                    n = 0;
                    loop {
                        si = si + 1;
                        match constructs.get(&src[si]) {
                            Some(&BFConstruct::Open) => n = n + 1,
                            Some(&BFConstruct::Close) => {
                                n = n - 1;
                                if n < 0 {
                                    break;
                                }
                            },
                            _ => print!("")
                        }
                    }
                }
            },
            Some(&BFConstruct::Close) => {
                if b[i] != 0 {
                    n = 0;
                    loop {
                        si -= 1;
                        match constructs.get(&src[si]) {
                            Some(&BFConstruct::Close) => n = n + 1,
                            Some(&BFConstruct::Open) => {
                                n = n - 1;
                                if n < 0 {
                                    break;
                                }
                            }
                            _ => print!("")
                        }
                    }
                }
            },
            _ => print!("")
        }
        si = si + 1;
        if si >= src.len() {
            break;
        }
    }
}