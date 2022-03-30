use std::{fs, io};
use crate::token;

type Span = (u32, u32);

pub struct Lexer {
    tokens: Vec<token::Token>,
    spans: Vec<Span>
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        Self
    }

    fn lex(&self) -> (Vec<token::Token>, Vec<Span>) {
        let tokens : Vec<token::Token> = vec![];
        let spans : Vec<Span> = vec![];
        (tokens, spans)
    }
}


