use rustyline::{
    error::ReadlineError, validate::MatchingBracketValidator, Completer, DefaultEditor, Helper,
    Highlighter, Hinter, Validator,
};

use crate::{
    interpreter::{default_env::default_env, eval},
    parser::Parser,
};
use std::io::{self, Write};

#[derive(Completer, Helper, Highlighter, Hinter, Validator)]
struct Validator {
    #[rustyline(Validator)]
    parens: MatchingBracketValidator,
}

impl Validator {
    fn new() -> Self {
        Self {
            parens: MatchingBracketValidator::new(),
        }
    }
}

pub fn repl() {
    let mut rl = DefaultEditor::new().expect("failed to create editor");
    rl.set_helper(Some(Validator::new()));
    let env = default_env();

    println!("Welcome to the Lust REPL!");
    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let ast = &Parser::new(&line).parse().unwrap();
                match eval(env.clone(), ast) {
                    Ok(v) => {
                        println!("{}", v)
                    }
                    Err(e) => panic!("{}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    // println!("Welcome to the Lust REPL!");
    // print!("> ");
    // io::stdout().flush().expect("failed to flush stdout");
    // let mut indent = 0;
    // let mut src = String::new();
    // let env = default_env();
    // loop {
    //     let mut parens = 0;
    //     io::stdin()
    //         .read_line(&mut src)
    //         .expect("failed to read line");

    //     src.chars().for_each(|c| {
    //         if c == '(' {
    //             parens += 1;
    //         } else if c == ')' {
    //             parens -= 1;
    //         }
    //     });
    //     if parens == 0 {
    //         let ast = &Parser::new(&src).parse().unwrap();
    //         // println!("Ast: {:?}", ast);
    //         match eval(env.clone(), ast) {
    //             Ok(v) => {
    //                 println!("{}", v)
    //             }
    //             Err(e) => panic!("{}", e),
    //         }
    //         src.clear();
    //     } else {
    //         print!("- ");
    //         io::stdout().flush().expect("failed to flush stdout");
    //         continue;
    //     }
    //     print!("\n> ");
    //     io::stdout().flush().expect("failed to flush stdout");
    // }
}
