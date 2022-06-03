mod token;
mod parser;
mod ast;
mod lex;
mod eval;
mod env;

use std::{env as fs_env, fs};
use crate::lex::lex;
use crate::parser::Parser;

fn main() {
    let dir = fs_env::current_dir().unwrap();
    let input = fs::read_to_string(format!(
        "{}/examples/simple.lir",
        dir.as_path().to_str().unwrap()
    )).expect("Something went wrong reading the file");

    // let mut stream = lex(input.as_str());
    // while let Some(t) = stream.next() {
    //     println!("{:?}", t)
    // }
    let mut ast = Parser::new(input.as_str()).parse();
    for n in ast {
        println!("{:?}", n);
    }
}
