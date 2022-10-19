use crate::{
    env::Env, 
    eval::eval, 
    lex::lex, 
    parse::parse,
    token::TokenStream
};
use std::io::{self, Write};

pub fn repl(env: &mut Env) {
    print!("Lust > ");
    io::stdout().flush().expect("failed to flush stdout");
    loop {
        let mut raw = String::new();
        io::stdin().read_line(&mut raw).expect("failed to read line");
        match eval(&parse(&mut TokenStream::new(lex(&raw)).peekable()), env) {
            Ok(r) => println!("{}", r),
            Err(e) => eprintln!("{}", e)
        }
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");
    }
}