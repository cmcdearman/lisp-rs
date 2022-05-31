mod token;
mod parse;
mod ast;
mod lex;

use std::{env, fs};
use std::path::Iter;
use logos::Logos;
use crate::lex::lex;
use crate::token::LogosToken;
use crate::parse::parse;

fn main() {
    let dir = env::current_dir().unwrap();
    let mut input = fs::read_to_string(format!(
        "{}/examples/simple.lir",
        dir.as_path().to_str().unwrap()
    )).expect("Something went wrong reading the file");

    // let mut stream = lex(input.as_str());
    // while let Some(t) = stream.next() {
    //     println!("{:?}", t)
    // }

    let mut ast = parse(&mut lex(input.as_str()), input.as_str());
    for n in ast {
        println!("{:?}", n);
    }
}
