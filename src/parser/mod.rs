use self::{ast::Expr, error::ParseResult, reader::sexpr::Sexpr};
use crate::parser::{error::ParserError, reader::sexpr::Atom};
use logos::{Lexer, Logos};
use num_bigint::BigInt;
use num_rational::{BigRational, Rational64};
use std::{
    fmt,
    ops::{Index, Range},
};

pub mod ast;
pub mod error;
pub mod reader;
mod tests;

// // Parser entry point
// pub fn expr(sexpr: &Sexpr) -> ParseResult<Expr> {
//     match sexpr {
//         Sexpr::Atom(a) => match a.clone() {
//             Atom::Lit(l) => match l.clone() {
//                 reader::sexpr::Lit::Int(n) => Ok(Expr::Lit(ast::Lit::Int(n))),
//                 reader::sexpr::Lit::Rational(r) => Ok(Expr::Lit(ast::Lit::Rational(r))),
//                 reader::sexpr::Lit::Real(f) => Ok(Expr::Lit(ast::Lit::Real(f))),
//                 reader::sexpr::Lit::Char(c) => Ok(Expr::Lit(ast::Lit::Char(c))),
//                 reader::sexpr::Lit::String(s) => Ok(Expr::Lit(ast::Lit::String(s))),
//             },
//             Atom::Symbol(s) => Ok(Expr::Symbol(s)),
//         },
//         Sexpr::Cons(c) => {
//             let mut iter = c.clone().into_iter();
//             if let Some(first) = iter.next() {
//                 let lam = match first {
//                     Sexpr::Atom(a) => match a {
//                         Atom::Symbol(s) => match &*s.to_string() {
//                             "lambda" => lambda(&mut iter),
//                             "let" => parse_let(&mut iter),
//                             "if" => parse_if(&mut iter),
//                             _ => parse_apply(&mut iter),
//                         },
//                         _ => Err(Error::new("cannot apply non-lambda")),
//                     },
//                     Sexpr::Cons(_) => parse_apply(&mut iter),
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
// }
