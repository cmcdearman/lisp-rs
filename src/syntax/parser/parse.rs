use chumsky::primitive::todo;

use super::{
    ast::{Decl, Lit, Root, Symbol},
    error::ParseResult,
};
use crate::{
    syntax::{
        parser::ast::Expr,
        reader::{
            sexpr::{self, Atom, Sexpr},
            token::Token,
        },
    },
    util::{node::SrcNode, span::Span},
};

pub fn parse<'src>(src: &'src str, root: &sexpr::Root) -> ParseResult<SrcNode<Root>> {
    // let mut decls = vec![];
    // for sexpr in root.0 {
    //     match sexpr {
    //         Sexpr::Atom(_) => todo!(),
    //         Sexpr::Pair(_) => todo!(),
    //         Sexpr::Nil => todo!(),
    //     }
    // }
    todo!()
    // Ok(SrcNode::new(Root { decls }))
}

fn parse_decl<'src>(src: &'src str, sexpr: &sexpr::Sexpr) -> ParseResult<SrcNode<Decl>> {
    todo!()
}

fn parse_expr<'src>(src: &'src str, sexpr: &sexpr::Sexpr) -> ParseResult<SrcNode<Expr>> {
    match sexpr.clone() {
        Sexpr::Atom(a) => match a.inner().clone() {
            Atom::Symbol(sym) => Ok(SrcNode::new(Expr::Symbol(Symbol(sym.clone())), a.span())),
            Atom::Number(num) => Ok(SrcNode::new(Expr::Lit(Lit::Number(num.clone())), a.span())),
            Atom::String(string) => Ok(SrcNode::new(
                Expr::Lit(Lit::String(string.clone())),
                a.span(),
            )),
        },
        Sexpr::Pair(p) => match p.inner().head() {
            Sexpr::Atom(a) => match a.inner().clone() {
                Atom::Symbol(sym) => match sym.as_ref() {
                    "if" => {
                        let mut iter = p.inner().tail().unwrap().into_iter();
                        let cond = parse_expr(src, &iter.next().unwrap())?;
                        let then = parse_expr(src, &iter.next().unwrap())?;
                        let else_ = parse_expr(src, &iter.next().unwrap())?;
                        Ok(SrcNode::new(Expr::If { cond, then, else_ }, p.span()))
                    }
                    "quote" => {
                        let mut iter = p.inner().tail().into_iter();
                        let sexpr = iter.next().unwrap();
                        Ok(SrcNode::new(
                            Expr::Quote(parse_expr(src, &sexpr)?),
                            p.span(),
                        ))
                    }
                    "unquote" => {
                        let mut iter = p.inner().tail().unwrap().into_iter();
                        let sexpr = iter.next().unwrap();
                        Ok(SrcNode::new(
                            Expr::Unquote(parse_expr(src, &sexpr)?),
                            p.span(),
                        ))
                    }
                    _ => {
                        // let mut iter = p.inner().into_iter();
                        // let head = parse_expr(src, &iter.next().unwrap())?;
                        // let args = iter
                        //     .map(|s| parse_expr(src, &s))
                        //     .collect::<ParseResult<_>>()?;
                        // Ok(SrcNode::new(Expr::Apply { fun: , args: () }, p.span()))
                        todo!()
                    }
                },
                _ => todo!(),
            },
            _ => todo!(),
        },
        s @ Sexpr::Nil => Ok(SrcNode::new(Expr::Nil, s.span())),
    }
}
