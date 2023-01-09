use crate::{
    ast::env::Env,
    eval::eval,
    lex::lex,
    parse::parse,
    token::{TokenKind, TokenStream},
};
use std::io::{self, Write};

pub fn repl(env: &mut Env) {
    print!("lust> ");
    io::stdout().flush().expect("failed to flush stdout");
    loop {
        let mut raw = String::new();
        io::stdin()
            .read_line(&mut raw)
            .expect("failed to read line");
        match eval(
            &parse(
                &mut TokenStream::new(
                    lex(&raw)
                        .into_iter()
                        .filter(|t| t.kind != TokenKind::Comment)
                        .collect(),
                )
                .peekable(),
            ),
            env,
        ) {
            Ok(obj) => println!("{}", obj),
            Err(e) => eprintln!("{}", e),
        }
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");
    }
}
