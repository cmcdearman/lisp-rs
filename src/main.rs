use parser::reader::token::TokenStream;

mod compiler;
mod intern;
mod list;
mod lower;
mod parser;
mod span;
mod vm;

fn main() {
    let tokens = TokenStream::new("(+ 1 2)").expect("Failed to lex tokens");
    let mut reader = parser::reader::Reader::new(tokens);
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    println!("{:?}", sexpr);
}
