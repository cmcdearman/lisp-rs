use parser::Reader;

mod compiler;
mod gc;
mod interner;
mod list;
mod lower;
mod parser;
mod vm;

fn main() {
    let reader = Reader::new("(+ 1 2)");
}
