use crate::{
    interpreter::{default_env::default_env, eval},
    parser::Parser,
};
use std::io::{self, Write};

pub fn repl() {
    println!("Welcome to the Lust REPL!");
    print!("> ");
    io::stdout().flush().expect("failed to flush stdout");
    let mut src = String::new();
    let env = default_env();
    loop {
        let mut parens = 0;
        io::stdin()
            .read_line(&mut src)
            .expect("failed to read line");

        src.chars().for_each(|c| {
            if c == '(' {
                parens += 1;
            } else if c == ')' {
                parens -= 1;
            }
        });
        if parens == 0 {
            let ast = &Parser::new(&src).parse().unwrap();
            // println!("Ast: {:?}", ast);
            match eval(env.clone(), ast) {
                Ok(v) => {
                    println!("{}", v)
                }
                Err(e) => panic!("{}", e),
            }
            src.clear();
        } else {
            print!("- ");
            io::stdout().flush().expect("failed to flush stdout");
            continue;
        }
        print!("\n> ");
        io::stdout().flush().expect("failed to flush stdout");
    }
}
