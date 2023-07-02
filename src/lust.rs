// use logos::Logos;

// use crate::intern::InternedString;
// use std::rc::Rc;

// #[derive(Logos, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
// pub enum TokenKind {
//     #[default]
//     Eof,
//     #[regex(r"[ \t\r\n\f]+", logos::skip)]
//     Whitespace,
//     #[regex(r#";[^\n]*"#)]
//     Comment,
//     #[regex(r#"[^\[\]()\s,{};]+"#)]
//     Ident,
//     #[regex(r#"([1-9]\d*|0)"#, priority = 3)]
//     Int,
//     #[regex(r#"(\+|-)?\d+/\d+"#)]
//     Rational,
//     #[regex(r#"((\d+(\.\d+))|(\.\d+))([Ee](\+|-)?\d+)?"#, priority = 2)]
//     Real,
//     #[regex(r#"'\w'"#)]
//     Char,
//     #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
//     String,
//     #[regex(r#"(true)|(false)"#)]
//     Bool,

//     #[token("(")]
//     LParen,
//     #[token(")")]
//     RParen,
//     #[token("[")]
//     LBrack,
//     #[token("]")]
//     RBrack,
//     #[token("{")]
//     LBrace,
//     #[token("}")]
//     RBrace,
//     #[token(":")]
//     Colon,
//     #[token(".")]
//     Period,
//     #[token(",")]
//     Comma,
//     #[token("#")]
//     Hash,
//     #[token("'")]
//     Quote,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// struct Span {
//     start: usize,
//     end: usize,
// }

// impl

// enum Object {
//     Atom(Atom),
//     List(List),
// }

// enum Atom {
//     Symbol(InternedString),
//     Int(i32),
//     Float(f64),
//     Bool(bool),
//     String(InternedString),
// }

// enum List {
//     Nil,
//     Cons(Rc<Cons>),
// }

// struct Cons {
//     car: Object,
//     cdr: Rc<Object>,
// }

// struct Reader<'src> {
//     lexer: logos::Lexer<'src, TokenKind>,
// }

// impl<'src> Reader<'src> {
//     fn new(src: &'src str) -> Self {
//         Self {
//             lexer: TokenKind::lexer(src),
//         }
//     }

//     fn peek(&self) -> Token {

//     }

//     fn next(&mut self) -> Option<char> {
//         self.chars.next()
//     }

//     fn skip_whitespace(&mut self) {
//         while let Some(ch) = self.peek() {
//             if !ch.is_whitespace() {
//                 break;
//             }
//             self.next();
//         }
//     }

//     fn read_atom(&mut self) -> Option<Object> {
//         match self.peek()? {
//             '"' => self.read_string(),
//             _ => self.read_symbol(),
//         }
//     }

//     fn read_list(&mut self) -> Option<Object> {
//         let mut list = Vec::new();
//         while let Some(ch) = self.peek() {
//             match ch {
//                 ')' => {
//                     self.next();
//                     break;
//                 }
//                 _ => {
//                     let obj = self.read()?;
//                     list.push(obj);
//                 }
//             }
//         }
//         Some(Object::List(List::Nil))
//     }

//     fn read(&mut self) -> Option<Object> {
//         self.skip_whitespace();
//         match self.peek()? {
//             '(' => self.read_list(),
//             _ => self.read_atom(),
//         }
//     }
// }
