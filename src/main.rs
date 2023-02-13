mod parser;
mod interpreter;
mod vm;

use interpreter::default_env;
use parser::Parser;

use crate::{
    interpreter::repl::repl,
};

fn main() {
    repl();
}