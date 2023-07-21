use std::env;

use crate::parser::{
    ast::{Expr, Lit},
    reader::token::TokenStream,
};

mod compiler;
mod intern;
mod list;
mod lower;
mod parser;
mod span;
mod vm;

fn main() {
    env_logger::init();
    let src = "123";
    let tokens = TokenStream::new(src).expect("Failed to lex tokens");
    let mut reader = parser::reader::Reader::new(tokens);
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    // println!("{:?}", sexpr);
    // let expr = (Expr::Lit(Lit::Int(123)), span::Span::from(0..3));
    let expr = parser::expr(&sexpr).expect("Failed to parse");
    let mut compiler = compiler::Compiler::new();
    let chunk = compiler.compile(&expr).expect("Failed to compile");
    println!("{:?}", chunk);
    let mut vm = vm::VM::new(chunk);
    let result = vm.run().expect("Runtime error");
    println!("{}", result);
}
