use crate::{ast::env::Env, eval::eval, lex::lex, parse::parse};
use std::io::{self, Write};

pub fn repl(env: &mut Env) {
    print!("Lust > ");
    io::stdout().flush().expect("failed to flush stdout");
    loop {
        let mut raw = String::new();
        io::stdin()
            .read_line(&mut raw)
            .expect("failed to read line");
        match eval(&parse(&mut lex(&raw)), env) {
            Ok(r) => println!("{}", r),
            Err(e) => eprintln!("{}", e),
        }
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");
    }
}
