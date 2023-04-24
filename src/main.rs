use reader::Reader;

mod compiler;
mod interner;
mod list;
mod parser;
mod reader;
mod sexpr;
mod token;
mod vm;

fn main() {
    let reader = Reader::new("(+ 1 2)");
}
