mod lexer;
mod token;

use std::{env, fs, str};
use crate::lexer::Lexer;
use crate::token::Token;

fn main() {
    let dir = env::current_dir().unwrap();
    let contents = fs::read_to_string(format!("{}\\examples\\simple.eli",
                                    dir.as_path().to_str().unwrap()))
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    let mut lex = Lexer::new(contents.as_str()).expect("Lexer error",);
    while let (Some(t), Some(s)) = lex.next() {
        println!("Token: {:?} Span: {:?}\n", t, s)
    };
}
