use lust_syntax::read::read;
use std::io::{self, Write};

pub fn repl() {
    let mut src = String::new();
    // let mut compiler = Compiler::default();
    // let mut vm = Interpreter::default();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut src)
            .expect("Failed to read line");
        match src.trim() {
            "exit" => break,
            _ => (),
        }
        match read(&src) {
            (Some(root), errs) => {
                println!("{}", root);
                if !errs.is_empty() {
                    println!("errs: {:?}", errs);
                }
            }
            (None, errs) => {
                println!("errs: {:?}", errs);
            }
        }
        io::stdout().flush().unwrap();
        src.clear();
    }
}
