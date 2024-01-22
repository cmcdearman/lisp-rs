use self::ast::*;
use crate::read::sexpr::{self, Sexpr};
use lust_utils::{list::List, span::Span};

pub mod ast;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    UnexpectedSexpr,
    UnexpectedEof,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn parse(root: sexpr::Root) -> (Option<ast::Root>, Vec<Error>) {
    let mut items = vec![];
    let mut errs = vec![];
    for sexpr in root.sexprs() {
        match parse_item(sexpr.clone()) {
            Ok(item) => items.push(item),
            Err(err) => errs.push(err),
        }
    }
    (Some(ast::Root::new(items, *root.span())), errs)
}

fn parse_item(sexpr: Sexpr) -> Result<Item> {
    match sexpr.kind() {
        sexpr::SexprKind::SynList(_) => todo!(),
        sexpr::SexprKind::Atom(_) | sexpr::SexprKind::DataList(_) | sexpr::SexprKind::Vector(_) => {
            Ok(Item::new(
                ItemKind::Expr(parse_expr(sexpr.clone())?),
                *sexpr.span(),
            ))
        }
    }
}

fn parse_decl(sexpr: Sexpr) -> Result<Decl> {
    todo!()
}

fn parse_expr(sexpr: Sexpr) -> Result<Expr> {
    match sexpr.kind() {
        sexpr::SexprKind::Atom(a) => match a.kind() {
            sexpr::AtomKind::Lit(l) => Ok(Expr::new(
                ExprKind::Lit(parse_lit(l.clone())?),
                *sexpr.span(),
            )),
            sexpr::AtomKind::Sym(name) => {
                Ok(Expr::new(ExprKind::Ident(name.clone()), *sexpr.span()))
            }
            sexpr::AtomKind::Path(_) => todo!(),
        },
        sexpr::SexprKind::SynList(_) => todo!(),
        sexpr::SexprKind::DataList(l) => {
            let mut new_list = List::Empty;
            for sexpr in l.list().iter() {
                println!("sexpr: {:?}", sexpr);
                new_list.push_front(parse_expr(sexpr.clone())?);
            }
            Ok(Expr::new(ExprKind::List(new_list), *sexpr.span()))
        }
        sexpr::SexprKind::Vector(v) => {
            let mut exprs = vec![];
            for sexpr in v.iter() {
                exprs.push(parse_expr(sexpr.clone())?);
            }
            Ok(Expr::new(ExprKind::Vector(exprs), *sexpr.span()))
        }
    }
}

fn parse_lit(lit: sexpr::Lit) -> Result<Lit> {
    match lit.clone() {
        sexpr::Lit::Int(i) => Ok(Lit::Int(i)),
        sexpr::Lit::Float(f) => Ok(Lit::Float(f)),
        sexpr::Lit::Rational(r) => Ok(Lit::Rational(r)),
        sexpr::Lit::Str(s) => Ok(Lit::Str(s)),
        sexpr::Lit::Bool(b) => Ok(Lit::Bool(b)),
        sexpr::Lit::Char(c) => Ok(Lit::Char(c)),
    }
}
