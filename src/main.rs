mod interpreter;
mod parser;

use interpreter::default_env;
use parser::Parser;

use crate::interpreter::repl::repl;

fn main() {
    repl();
}
