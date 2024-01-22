use lust_syntax::{
    expand::{expand_macros, store::Store},
    read::read, parse::parse,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, Write},
    rc::Rc,
};

pub fn repl() {
    let mut src = String::new();
    // let mut compiler = Compiler::default();
    // let mut vm = Interpreter::default();
    let store = Store::new();
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
        // for m in collect_macros(&root) {
        //     println!("{}", m);
        // }
        // let expanded = expand_macros(store.clone(), &root);
        // println!("expanded: {:#?}", expanded);
        if let (Some(ast), errors) = parse(root) {
            println!("ast: {:#?}", ast);
            if !errors.is_empty() {
                println!("errors: {:?}", errors);
                continue;
            }
        }
        io::stdout().flush().unwrap();
        src.clear();
    }
}
