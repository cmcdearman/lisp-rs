mod ast;
mod default_env;
mod eval;
mod lex;
mod parse;
mod repl;
mod token;

use default_env::default_env;

use crate::{repl::repl, token::{TokenStream, TokenKind}};

// fn main() {
//     repl(&mut default_env());
// }

fn main() {
    let input = "(+ 3 4)";

    let ast = parse::parse(&mut TokenStream::new(
                    lex::lex(&input)
                        .into_iter()
                        .filter(|t| t.kind != TokenKind::Comment)
                        .collect(),
                )
                .peekable());

    println!("{}", ast);
}
