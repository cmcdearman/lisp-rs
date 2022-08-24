mod token;
mod parser;
mod ast;
mod lex;
mod eval;
mod env;

use std::{env as fs_env, fs};
use crate::lex::lex;
use crate::parser::parse;

fn main() {
    // let dir = fs_env::current_dir().unwrap();
    // let input = fs::read_to_string(format!(
    //     "{}/examples/deadSimple.lir",
    //     dir.as_path().to_str().unwrap()
    // )).expect("Something went wrong reading the file");

    // let input = "(+ (/ 6 (* 1.5 2)) (- 1 (mod 5 2))) ; 2.0";
    let input = "(1 2 3)";
    let mut tokens = lex(input);
    for t in tokens {
       println!("{:?}", t);
    }

    // let ast = Parser::new(
    //     format!("({})", input).as_str())
    //     .parse();
    // println!("{:?}", ast);
}
