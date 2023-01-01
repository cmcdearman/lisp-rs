mod ast;
mod default_env;
mod eval;
mod lex;
mod parse;
mod repl;
mod token;

use default_env::default_env;

use crate::repl::repl;

fn main() {
    repl(&mut default_env());
}
