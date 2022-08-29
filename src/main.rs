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

    /*
     * (+ (/ 6 (* 1.5 2)) (- 1 (mod 5 2))) ; 2.0
     *
     * =============== AST ===============
     *
     * [
     *  0 Sexpr::Nil,
     *  1 Sexpr::Nil,
     *  2 Sexpr::Nil,
     *  3 Sexpr::Num(2.0),
     *  4 Sexpr::Cons(3, 2),
     *  5 Sexpr::Num(5.0)
     *  6 Sexpr::Cons(5, 4)
     *  7 Sexpr::Sym("mod"),
     *  8 Sexpr::Cons(7, 6)
     *  9
     * ]
     *
     */

    /*
     * (+ 1 (- 2 3))
     * (+ . (1.0 . ((- . (2.0 . (3.0 . null))) . null)))
     *
     * 0  Sexpr::Nil,
     * 1  Sexpr::Nil,
     * 2  Sexpr::Num(3.0),
     * 3  Sexpr::Cons(2, 1),
     * 4  Sexpr::Num(2.0),
     * 5  Sexpr::Cons(4, 3),
     * 6  Sexpr::Sym("-"),
     * 7  Sexpr::Cons(6, 5),
     * 8  Sexpr::Cons(7, 0),
     * 9  Sexpr::Num(1.0),
     * 10 Sexpr::Cons(9, 8),
     * 11 Sexpr::Sym("+"),
     * 12 Sexpr::Cons(11, 10)
     */

    // LET input = "(+ (/ 6 (* 1.5 2)) (- 1 (mod 5 2))) ; 2.0";
    let input = "(1 2 3)";
    let mut tokens = lex(input);
    for t in &tokens {
       println!("{:?}", t);
    }

    let ast = parse(tokens);
    println!("{:?}", ast);
}
