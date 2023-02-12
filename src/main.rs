mod sexpr;
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

// fn main() {
//     let input = "(+ 3 4)";

//     let tokens = TokenStream::new(
//         lex::lex(&input)
//             .into_iter()
//             .filter(|t| t.kind != TokenKind::Comment)
//             .collect(),
//     );

//     let ast = parse::parse(&mut tokens.clone().peekable());

//     for t in tokens {
//         println!("{:?} {}", t, t.lit)
//     }
//     println!("{}", ast);
// }
