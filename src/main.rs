mod ast;
mod env;
mod eval;
mod lex;
mod parse;
mod token;

use crate::lex::lex;
use crate::parse::parse;
use crate::token::TokenStream;
use std::{env as fs_env, fs};

fn main() {
    // let dir = fs_env::current_dir().unwrap();
    // let input = fs::read_to_string(format!(
    //     "{}/examples/deadSimple.lir",
    //     dir.as_path().to_str().unwrap()
    // )).expect("Something went wrong reading the file");

    // let input = "(+ (/ 6 (* 1.5 2)) (- 1 (mod 5 2))) ; 2.0";
    let input = "(* (+ 2 3) 4)";
    let mut tokens = lex(input);
    // for t in &tokens {
    //     println!("{:?}", t);
    // }

    let mut ast = Vec::new();

    parse(&mut TokenStream::new(tokens).peekable(),
        &mut |expr| { ast.push(expr); (ast.len() - 1) as u32}, );

    for (i, node) in ast.iter().enumerate() {
        println!("{} => {:?}", i, node);
    }
    // println!("{:?}", ast);
}
