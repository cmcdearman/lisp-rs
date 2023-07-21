use self::{ast::Expr, error::ParseResult, reader::sexpr::Sexpr};
use crate::parser::{error::Error, reader::sexpr::Atom};
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
//             Atom::Int(n) => Ok(Expr::Lit(ast::Lit::Int(n))),
//             Atom::Real(f) => Ok(Expr::Lit(ast::Lit::Real(f))),
//             Atom::Rational(r) => Ok(Expr::Lit(ast::Lit::Rational(r))),
//             Atom::Bool(b) => Ok(Expr::Lit(ast::Lit::Bool(b))),
//             Atom::Char(c) => Ok(Expr::Lit(ast::Lit::Char(c))),
//             Atom::String(s) => Ok(Expr::Lit(ast::Lit::String(s))),
//             Atom::Symbol(s) => Ok(Expr::Symbol(s)),
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
//                         _ => Err(Error::new("cannot apply non-lambda")),
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
