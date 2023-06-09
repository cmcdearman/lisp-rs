// use std::{
//     fmt,
//     ops::{Index, Range},
// };

// use logos::{Lexer, Logos};
// use num_bigint::BigInt;
// use num_rational::{BigRational, Rational64};

// use crate::{interner::InternedString, list::List, parser::reader::Atom};

// use self::reader::Sexpr;

pub mod reader;
mod tests;

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct ParserError(pub InternedString);

// impl ParserError {
//     pub fn new(msg: &str) -> Self {
//         Self(msg.into())
//     }
// }

// impl Display for ParserError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", InternedString::from(self.0.key))
//     }
// }

// pub type ParseResult<T> = std::result::Result<T, ParserError>;

// #[derive(Debug, Clone, PartialEq)]
// pub enum Expr {
//     Lit(Lit),
//     List(List<Self>),
//     Lambda {
//         param: Vec<Self>,
//         body: Box<Self>,
//     },
//     Apply {
//         func: Box<Self>,
//         arg: Box<Self>,
//     },
//     Let {
//         name: InternedString,
//         value: Box<Self>,
//         body: Box<Self>,
//     },
//     If {
//         cond: Box<Self>,
//         then: Box<Self>,
//         else_: Box<Self>,
//     },
//     Unit,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum Lit {
//     Int(i64),
//     Real(f64),
//     Rational(Rational64),
//     Bool(bool),
//     Char(char),
//     String(InternedString),
//     Symbol(InternedString),
//     Record(Record),
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Record {
//     name: InternedString,
//     fields: Vec<InternedString>,
// }

// // Parser entry point
// pub fn expr(sexpr: &Sexpr) -> ParseResult<Expr> {
//     match sexpr {
//         Sexpr::Atom(a) => match a.clone() {
//             Atom::Int(n) => Ok(Expr::Lit(Lit::Int(n))),
//             Atom::Real(f) => Ok(Expr::Lit(Lit::Real(f))),
//             Atom::Rational(r) => Ok(Expr::Lit(Lit::Rational(r))),
//             Atom::Bool(b) => Ok(Expr::Lit(Lit::Bool(b))),
//             Atom::Char(c) => Ok(Expr::Lit(Lit::Char(c))),
//             Atom::String(s) => Ok(Expr::Lit(Lit::String(s))),
//             Atom::Symbol(s) => Ok(Expr::Lit(Lit::Symbol(s))),
//         },
//         Sexpr::List(l) => {
//             let mut iter = l.clone().into_iter();
//             if let Some(first) = iter.next() {
//                 let lam = match first {
//                     Sexpr::Atom(a) => match a {
//                         Atom::Symbol(s) => match &*s.to_string() {
//                             "lambda" => lambda(&mut iter),
//                             "let" => parse_let(&mut iter),
//                             "if" => parse_if(&mut iter),
//                             _ => parse_apply(&mut iter),
//                         },
//                         _ => Err(ParserError::new("cannot apply non-lambda")),
//                     },
//                     Sexpr::List(_) => parse_apply(&mut iter),
//                 }?;
//             } else {
//                 Ok(Expr::Unit)
//             }
//         }
//     }

//     fn lambda(list_iter: &mut ConsIter) -> ParseResult<Expr> {
//         let params = list_iter
//             .next()
//             .ok_or(ParserError::new("lambda missing parameter list"))?;
//         let body = list_iter
//             .next()
//             .ok_or(ParserError::new("lambda missing body"))?;
//         Ok(Expr::Lambda {
//             param: params,
//             body: Box::new(expr(body)?),
//         })
//     }

//     fn curry_fn(mut params: impl Iterator<Item = InternedString>, body: Expr) -> ParseResult<Expr> {
//         Ok(params.fold(body, |acc, p| {
//             Expr::Lit(Lit::Lambda {
//                 param: p,
//                 body: Box::new(acc),
//             })
//         }))
//     }

//     fn curry_apply(&mut self, args: impl Iterator<Item = Expr>, func: Expr) -> Expr {
//         args.fold(func, |acc, arg| Expr::Apply(Box::new(acc), Box::new(arg)))
//     }
// }
