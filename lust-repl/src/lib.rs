use lust_syntax::{expand::collect_macros, read::read};
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
        let root = match read(&src) {
            (Some(root), errs) => {
                // println!("{}", root);
                if !errs.is_empty() {
                    println!("errs: {:?}", errs);
                    continue;
                }
                root
            }
            (None, errs) => {
                println!("errs: {:?}", errs);
                continue;
            }
        };
        // println!("macros: {:#?}", collect_macros(&root));
        for m in collect_macros(&root) {
            println!("{}", m);
        }
        io::stdout().flush().unwrap();
        src.clear();
    }
}
