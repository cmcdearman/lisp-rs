mod ast;
mod env;
mod eval;
mod lex;
mod parse;
mod repl;
mod token;

use crate::repl::repl;

fn main() {
    repl(&mut env::default_env());
}
