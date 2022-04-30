extern crate core;

mod lexer;
mod parser;
mod token;
mod ast;

use crate::lexer::Lexer;
use std::{env, fs};

fn main() {
    let dir = env::current_dir().unwrap();
    let contents = fs::read_to_string(format!(
        "{}/examples/simple.eli",
        dir.as_path().to_str().unwrap()
    ))
    .expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);

    let mut lex = Lexer::new(contents.as_str());
    while let (Some(t), Some(s)) = lex.next() {
        println!("Token: {:?} Span: {:?}", t, s);
    }
    // let ast = parser::parse(&mut lex);
    // println!("Ast: {:?}", ast)
}
