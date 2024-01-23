use self::ast::*;
use crate::read::sexpr::{self, Sexpr, SexprKind};
use lust_utils::{list::List, span::Span};
use num_bigfloat::E;

pub mod ast;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    msg: String,
    span: Span,
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum ErrorKind {
//     UnexpectedSexpr,
//     UnexpectedEof,
// }

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
        sexpr::SexprKind::SynList(l) => {
            let mut iter = l.list().iter();
            if let Some(first) = iter.next() {
                if let Some(atom) = first.as_atom() {
                    if let Some(sym) = atom.as_sym() {
                        match sym.as_ref() {
                            "def" => {
                                let name = match iter.next() {
                                    Some(name) => name,
                                    None => {
                                        return Err(Error {
                                            msg: "expected name".to_string(),
                                            span: *sexpr.span(),
                                        })
                                    }
                                };
                                let expr = match iter.next() {
                                    Some(expr) => expr,
                                    None => {
                                        return Err(Error {
                                            msg: "expected expr".to_string(),
                                            span: *sexpr.span(),
                                        })
                                    }
                                };
                                Ok(Item::new(
                                    ItemKind::Decl(Decl::new(
                                        DeclKind::Def {
                                            name: name.clone(),
                                            expr: parse_expr(expr.clone())?,
                                            span: sexpr.span().clone(),
                                        },
                                        *sexpr.span(),
                                    )),
                                    *sexpr.span(),
                                ))
                            }
                        }
                    } else {
                        return Err(Error {
                            msg: "expected symbol".to_string(),
                            span: *sexpr.span(),
                        });
                    }
                } else {
                    return Err(Error {
                        msg: "expected atom".to_string(),
                        span: *sexpr.span(),
                    });
                }
            } else {
                return Err(Error {
                    msg: "expected atom".to_string(),
                    span: *sexpr.span(),
                });
            }
        }
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
            let mut exprs = vec![];
            for sexpr in l.list().iter() {
                exprs.push(parse_expr(sexpr.clone())?);
            }
            Ok(Expr::new(ExprKind::List(List::from(exprs)), *sexpr.span()))
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
