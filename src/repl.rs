use crate::{
    object::env::Env,
    eval::eval,
    lex::lex,
    parse::parse,
    token::{TokenKind, TokenStream},
};
use std::{
    io::{self, Write}, rc::Rc, cell::RefCell,
};

pub fn repl(env: Env) {
    let env_rc = Rc::new(RefCell::new(env));
    print!("lust> ");
    io::stdout().flush().expect("failed to flush stdout");
    loop {
        let mut raw = String::new();
        io::stdin()
            .read_line(&mut raw)
            .expect("failed to read line");
        match eval(
            match &parse(
                &mut TokenStream::new(
                    lex(&raw)
                        .into_iter()
                        .filter(|t| t.kind != TokenKind::Comment)
                        .collect(),
                )
                .peekable(),
            ) {
                Ok(ast) => ast,
                Err(err) => panic!("{}", err),
            },
            env_rc.clone(),
        ) {
            Ok(obj) => println!("{}", obj),
            Err(e) => eprintln!("{}", e),
        }
        print!("\nlust> ");
        io::stdout().flush().expect("failed to flush stdout");
    }
}
