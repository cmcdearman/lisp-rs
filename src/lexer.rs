use std::fmt::{Error};
use std::iter::Peekable;
use std::vec::IntoIter;
use crate::token::Token;

type Span = (usize, usize);

#[derive(Debug)]
pub struct Lexer {
    tokens: Peekable<IntoIter<Token>>,
    spans: Peekable<IntoIter<Span>>,
}

pub struct LexerError;

impl Lexer {
    pub fn new(src: &str) -> Self {
        let mut tokens : Vec<Token> = vec![];
        let mut spans : Vec<Span> = vec![];

        let mut s = src.chars().peekable();
        // Lex tokens

        let mut pos: usize = 0;
        while let Some(&c) = s.peek() {
            // println!("char: {}", c);
            match c {
                ' ' | '\t' | '\n' | '\r' => { s.next(); }
                '0'..='9' => {
                    tokens.push(read_number(&mut s));
                }
                '+' => {
                    tokens.push(Token::ADD(c));
                    s.next();
                }
                '-' => {
                    tokens.push(Token::SUB(c));
                    s.next();
                }
                '*' => {
                    tokens.push(Token::MUL(c));
                    s.next();
                }
                '/' => {
                    tokens.push(Token::QUO(c));
                    s.next();
                }
                '%' => {
                    tokens.push(Token::MOD(c));
                    s.next();
                }
                '(' => {
                    tokens.push(Token::LPAREN(c));
                    s.next();
                }
                ')' => {
                    tokens.push(Token::RPAREN(c));
                    s.next();
                }
                _ => {
                    if c.is_alphabetic() {
                        tokens.push(read_ident(&mut s));
                        s.next();
                    } else {
                        tokens.push(Token::ILLEGAL(String::from(c)));
                        s.next();
                    }
                }
            }
        }
        Self { tokens: tokens.into_iter().peekable(), spans: spans.into_iter().peekable()}
    }

    pub fn next(&mut self) -> (Option<Token>, Option<Span>) {
        (self.tokens.next(), self.spans.next())
    }

    pub fn peek(&mut self) -> (Option<&Token>, Option<&Span>) {
        (self.tokens.peek(), self.spans.peek())
    }
}

fn read_ident<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> Token {
    let mut ident = iter.next().unwrap().to_string();
    while let Some(ch) = iter.peek() {
        if ch.is_whitespace() {
            return match &*ident {
                "let" => { Token::LET }
                _ => { Token::IDENT((&*ident).parse().unwrap()) }
            }
        }
        if ch.is_ascii_alphanumeric() {
            ident.push(*ch);
            iter.next();
        }
    }
    Token::EOF
}

fn read_number<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> Token {
    let mut number = iter.next().unwrap().to_string().parse::<i32>().expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<i32>()) {
        number = number * 10 + digit;
        iter.next();
    }
    Token::INT(number)
}
