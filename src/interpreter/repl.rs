use rustyline::{
    error::ReadlineError, highlight::MatchingBracketHighlighter,
    validate::MatchingBracketValidator, Completer, DefaultEditor, Editor, Helper, Highlighter,
    Hinter, Validator,
};

use crate::{
    interpreter::{default_env::default_env, eval},
    parser::Parser,
};
use std::io::{self, Write};

#[derive(Completer, Helper, Highlighter, Hinter, Validator)]
struct ReplHelper {
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Highlighter)]
    highlighter: MatchingBracketHighlighter,
}

impl ReplHelper {
    fn new() -> Self {
        Self {
            validator: MatchingBracketValidator::new(),
            highlighter: MatchingBracketHighlighter::new(),
        }
    }
}

pub fn repl() {
    let mut rl = Editor::new().expect("failed to create editor");
    rl.set_helper(Some(ReplHelper::new()));
    let env = default_env();

    println!("Welcome to the Lust REPL!");
    loop {
        match rl.readline("> ") {
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
}
