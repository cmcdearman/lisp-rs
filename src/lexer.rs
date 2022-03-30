use std::{fs, io};
use std::fmt::Error;
use crate::token;

type Span = (u32, u32);

pub struct Lexer {
    tokens: Vec<token::Token>,
    spans: Vec<Span>
}

pub struct LexerError;

impl Lexer {
    pub fn new(src: &str) -> Result<Self, Error> {
        let tokens : Vec<token::Token> = vec![];
        let spans : Vec<Span> = vec![];
        Ok(Self { tokens, spans })
    }

    pub fn next(self) -> (token::Token, Span) {
        todo!()
    }
}


