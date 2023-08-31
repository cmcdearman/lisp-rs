use crate::{ast::Expr, error::ParseResult, reader::sexpr::Root};
use logos::{Lexer, Logos};
use lust_util::span::Spanned;
use num_bigint::BigInt;
use num_rational::{BigRational, Rational64};
use std::{
    fmt,
    ops::{Index, Range},
};

pub fn parse(sexprs: &Root) -> ParseResult<Spanned<Expr>> {
    todo!()
}

// Parser entry point
// pub fn expr(sexpr: &Spanned<Sexpr>) -> ParseResult<Spanned<Expr>> {
//     match sexpr.0.clone() {
//         Sexpr::Atom(a) => match a.clone() {
//             Atom::Lit(l) => match l.clone() {
//                 reader::sexpr::Lit::Int(n) => Ok((Expr::Lit(ast::Lit::Int(n)), sexpr.1)),
//                 reader::sexpr::Lit::Rational(r) => Ok((Expr::Lit(ast::Lit::Rational(r)), sexpr.1)),
//                 reader::sexpr::Lit::Real(f) => Ok((Expr::Lit(ast::Lit::Real(f)), sexpr.1)),
//                 reader::sexpr::Lit::Char(c) => Ok((Expr::Lit(ast::Lit::Char(c)), sexpr.1)),
//                 reader::sexpr::Lit::String(s) => Ok((Expr::Lit(ast::Lit::String(s)), sexpr.1)),
//             },
//             Atom::Symbol(s) => Ok((Expr::Symbol(s), sexpr.1)),
//         },
//         Sexpr::Cons(c) => {
//             todo!()
//             // let mut iter = c.clone().into_iter();
//             // if let Some(first) = iter.next() {
//             //     let lam = match first {
//             //         Sexpr::Atom(a) => match a {
//             //             Atom::Symbol(s) => match &*s.to_string() {
//             //                 "lambda" => lambda(&mut iter),
//             //                 "let" => parse_let(&mut iter),
//             //                 "if" => parse_if(&mut iter),
//             //                 _ => parse_apply(&mut iter),
//             //             },
//             //             _ => Err(Error::new("cannot apply non-lambda")),
//             //         },
//             //         Sexpr::Cons(_) => parse_apply(&mut iter),
//             //     }?;
//             // } else {
//             //     Ok(Expr::Unit)
//             // }
//         }
//     }

//     // fn lambda(list_iter: &mut ConsIter) -> ParseResult<Expr> {
//     //     let params = list_iter
//     //         .next()
//     //         .ok_or(ParserError::new("lambda missing parameter list"))?;
//     //     let body = list_iter
//     //         .next()
//     //         .ok_or(ParserError::new("lambda missing body"))?;
//     //     Ok(Expr::Lambda {
//     //         param: params,
//     //         body: Box::new(expr(body)?),
//     //     })
//     // }
// }
