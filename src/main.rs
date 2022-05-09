mod lexer;
// mod parser;
mod token;
// mod ast;

use std::{env, fs};
use crate::lexer::Lexer;

fn main() {
    let dir = env::current_dir().unwrap();
    let input = fs::read_to_string(format!(
        "{}/examples/simple.eli",
        dir.as_path().to_str().unwrap()
    ))
    .expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);

    let mut lexer = Lexer::new();
    let mut tokens = lexer.tokenize(input.as_str());
    for t in tokens {
        println!("{}", t);
    }
    // let ast = parser::parse(&mut lex);
    // println!("Ast: {:?}", ast)
}
