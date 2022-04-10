use crate::token::Token;

use std::iter::Peekable;
use std::vec::IntoIter;

type Span = (usize, usize);

#[derive(Debug)]
pub struct Lexer {
    tokens: Peekable<IntoIter<Token>>,
    spans: Peekable<IntoIter<Span>>,
}

pub struct LexerError;

impl Lexer {
    pub fn new(src: &str) -> Self {
        let mut tokens: Vec<Token> = vec![];
        let spans: Vec<Span> = vec![];

        let mut s = src.chars().peekable();
        // Lex tokens

        let _pos: usize = 0;
        while let Some(&c) = s.peek() {
            // println!("char: {}", c);
            match c {
                ' ' | '\t' | '\n' | '\r' => {
                    s.next();
                }
                '0'..='9' => {
                    tokens.push(read_number(&mut s));
                }
                '+' => {
                    tokens.push(Token::Add(c));
                    s.next();
                }
                '-' => {
                    tokens.push(Token::Sub(c));
                    s.next();
                }
                '*' => {
                    tokens.push(Token::Mul(c));
                    s.next();
                }
                '/' => {
                    tokens.push(Token::Quo(c));
                    s.next();
                }
                '%' => {
                    tokens.push(Token::Mod(c));
                    s.next();
                }
                '(' => {
                    tokens.push(Token::LParen(c));
                    s.next();
                }
                ')' => {
                    tokens.push(Token::RParen(c));
                    s.next();
                }
                _ => {
                    if c.is_alphabetic() {
                        tokens.push(read_ident(&mut s));
                        s.next();
                    } else {
                        tokens.push(Token::Illegal(String::from(c)));
                        s.next();
                    }
                }
            }
        }
        Self {
            tokens: tokens.into_iter().peekable(),
            spans: spans.into_iter().peekable(),
        }
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
                "let" => Token::Let,
                _ => Token::Ident((&*ident).parse().unwrap()),
            };
        }
        if ch.is_ascii_alphanumeric() {
            ident.push(*ch);
            iter.next();
        }
    }
    Token::Eof
}

fn read_number<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> Token {
    let mut number = iter
        .next()
        .unwrap()
        .to_string()
        .parse::<i32>()
        .expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<i32>()) {
        number = number * 10 + digit;
        iter.next();
    }
    Token::Int(number)
}
