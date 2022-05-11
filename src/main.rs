mod token;
mod lexer;
mod parser;

use std::{env, fs};
use crate::lexer::Lexer;

fn main() {
    let dir = env::current_dir().unwrap();
    let mut input = fs::read_to_string(format!(
        "{}/examples/simple.eli",
        dir.as_path().to_str().unwrap()
    ))
    .expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);

    let mut lexer = Lexer::new(input.as_str());
    let mut tokens = lexer.tokenize();
    for t in tokens {
        println!("{} {:?}", t.text(input.as_str()), t);
    }
    // let ast = parser::parse(&mut lex);
    // println!("Ast: {:?}", ast)
}
