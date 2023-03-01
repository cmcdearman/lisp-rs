mod interpreter;
mod parser;

use std::fmt::format;

use interpreter::default_env;
use itertools::join;
use parser::Parser;

use crate::interpreter::repl::repl;

fn main() {
    repl();
}
