use parser::reader::token::TokenStream;

mod intern;
mod list;
mod lower;
mod parser;
mod span;
mod vm;

fn main() {
    let tokens = TokenStream::new("1");
}
