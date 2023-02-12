use crate::{
    parser::{self, Parser},
    sexpr::env::Env,
};
use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
};

pub fn repl() {
    // let env_rc = Rc::new(RefCell::new(env));
    print!("lust> ");
    io::stdout().flush().expect("failed to flush stdout");
    loop {
        let mut src = String::new();
        io::stdin()
            .read_line(&mut src)
            .expect("failed to read line");
        match Parser::new(&src, false).sexpr() {
            Ok(ast) => println!("{:?}", ast),
            Err(err) => panic!("{}", err),
        }
        // env_rc.clone(),
        print!("\nlust> ");
        io::stdout().flush().expect("failed to flush stdout");
    }
}
