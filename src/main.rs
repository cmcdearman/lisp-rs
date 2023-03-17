mod interpreter;
mod parser;

use crate::interpreter::repl::repl;

fn main() {
    repl();
}
