// use crate::token::Token;

// use std::iter::{Enumerate, Peekable};
// use std::ops::Range;
// use std::vec::IntoIter;

// type Span = Range<usize>;

// #[derive(Debug)]
// pub struct Lexer {
//     tokens: Peekable<IntoIter<Token>>,
//     spans: Peekable<IntoIter<Span>>,
// }

// impl Lexer {
//     pub fn new(src: &str) -> Self {
//         let mut tokens: Vec<Token> = vec![];
//         let mut spans: Vec<Span> = vec![];

//         let mut s = src.chars().enumerate().peekable();

//         while let Some(&(index, c)) = s.peek() {
//             match c {
//                 ' ' | '\t' | '\n' | '\r' => {
//                     s.next();
//                 }
//                 '0'..='9' => {
//                     let num = read_number(&mut s);
//                     tokens.push(num.0);
//                     spans.push(num.1);
//                 }
//                 '+' => {
//                     tokens.push(Token::Add);
//                     spans.push(index..index+1);
//                     s.next();
//                 }
//                 '-' => {
//                     tokens.push(Token::Sub);
//                     spans.push(index..index+1);
//                     s.next();
//                 }
//                 '*' => {
//                     tokens.push(Token::Mul);
//                     spans.push(index..index+1);
//                     s.next();
//                 }
//                 '/' => {
//                     tokens.push(Token::Quo);
//                     spans.push(index..index+1);
//                     s.next();
//                 }
//                 '%' => {
//                     tokens.push(Token::Mod);
//                     spans.push(index..index+1);
//                     s.next();
//                 }
//                 '(' => {
//                     tokens.push(Token::LParen);
//                     spans.push(index..index+1);
//                     s.next();
//                 }
//                 ')' => {
//                     tokens.push(Token::RParen);
//                     spans.push(index..index+1);
//                     s.next();
//                 }
//                 _ => {
//                     if c.is_alphabetic() {
//                         let ident = read_ident(&mut s);
//                         tokens.push(ident.0);
//                         spans.push(ident.1);
//                         s.next();
//                     } else {
//                         tokens.push(Token::Illegal(String::from(c)));
//                         s.next();
//                     }
//                 }
//             }
//         }
//         tokens.push(Token::Eof);
//         spans.push(spans.last().map(|s| s.end + 1..s.end + 1).unwrap());
//         Self {
//             tokens: tokens.into_iter().peekable(),
//             spans: spans.into_iter().peekable(),
//         }
//     }

//     pub fn next(&mut self) -> (Option<Token>, Option<Span>) {
//         (self.tokens.next(), self.spans.next())
//     }

//     pub fn peek(&mut self) -> (Option<&Token>, Option<&Span>) {
//         (self.tokens.peek(), self.spans.peek())
//     }
// }

// fn read_ident<T: Iterator<Item = char>>(iter: &mut Peekable<Enumerate<T>>) -> (Token, Span) {
//     let next = iter.next().unwrap();
//     let start = next.0;
//     let mut ident = next.1.to_string();
//     while let Some((i, ch)) = iter.peek() {
//         if ch.is_whitespace() {
//             return match &*ident {
//                 "let" => (Token::Let, start..*i),
//                 _ => (Token::Ident((&*ident).parse().unwrap()), start..*i),
//             };
//         }
//         if ch.is_ascii_alphanumeric() {
//             ident.push(*ch);
//             iter.next();
//         }
//     }
//     (Token::Eof, start..start)
// }

// fn read_number<T: Iterator<Item = char>>(iter: &mut Peekable<Enumerate<T>>) -> (Token, Span) {
//     let next = iter.next().unwrap();
//     let start = next.0;
//     let mut end = start;
//     let mut n = next.1
//         .to_string()
//         .parse::<i32>()
//         .expect("The caller should have passed a digit.");
//     while let Some((i, Ok(digit))) = iter.peek().map(|c| (c.0, c.1.to_string().parse::<i32>())) {
//         n = n * 10 + digit;
//         end = i;
//         iter.next();
//     }
//     (Token::Number(n), start..end+1)
// }
