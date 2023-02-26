use crate::{
    interpreter::{default_env::default_env, eval},
    parser::Parser,
};
use std::io::{self, Write};

pub fn repl() {
    // let env_rc = Rc::new(RefCell::new(env));
    print!("lust> ");
    io::stdout().flush().expect("failed to flush stdout");
    let env = default_env();
    loop {
        let mut src = String::new();
        io::stdin()
            .read_line(&mut src)
            .expect("failed to read line");
        // match Parser::new(&src, false).parse() {
        //     Ok(ast) => println!("{:?}", ast),
        //     Err(err) => panic!("{}", err),
        // }
        let ast = &Parser::new(&src).parse().unwrap();
        // println!("Ast: {:?}", ast);
        match eval(env.clone(), ast) {
            Ok(v) => {
                println!("{}", v)
            }
            Err(e) => panic!("{}", e),
        }
        // env_rc.clone(),
        print!("\nlust> ");
        io::stdout().flush().expect("failed to flush stdout");
    }
}
