use std::{fs, io};
use std::fmt::{Error, format};
use std::iter::Peekable;
use std::vec::IntoIter;
use crate::token;
use crate::token::Token;

type Span = (usize, usize);

pub struct Lexer {
    tokens: Peekable<IntoIter<token::Token>>,
    spans: Peekable<IntoIter<Span>>,
}

pub struct LexerError;

impl Lexer {
    pub fn new(src: &str) -> Result<Self, Error> {
        let mut tokens : Vec<token::Token> = vec![];
        let mut spans : Vec<Span> = vec![];

        let mut s = src.chars().peekable();
        // Lex tokens
        while let Some(&c) = s.peek() {
            match c {
                '0'..='9' => {
                    let n = read_number(c, &mut s);
                    tokens.push(Token::INT(n));
                }
                '+' => {
                    tokens.push(Token::ADD(c))
                }
                '-' => {
                    tokens.push(Token::SUB(c))
                }
                '*' => {
                    tokens.push(Token::MUL(c))
                }
                '/' => {
                    tokens.push(Token::QUO(c))
                }
                '%' => {
                    tokens.push(Token::MOD(c))
                }
                '(' => {
                    tokens.push(Token::LPAREN(c))
                }
                ')' => {
                    tokens.push(Token::RPAREN(c))
                }
                _ => {
                    if c.is_alphabetic() {

                    } else {
                        tokens.push(Token::ILLEGAL(String::from(c)));
                        // Err(format!("Illegal character: {}", c));
                    }
                }
            }
        }

        Ok(Self { tokens: tokens.into_iter().peekable(), spans: spans.into_iter().peekable()})
    }

    pub fn next(&mut self) -> (Option<token::Token>, Option<Span>) {
        (self.tokens.next(), self.spans.next())
    }

    pub fn peek(&mut self) -> (Option<&token::Token>, Option<&Span>) {
        (self.tokens.peek(), self.spans.peek())
    }
}

fn read_ident() -> String { todo!() }

fn read_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> i32 {
    let mut number = c.to_string().parse::<i32>().expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<i32>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}



