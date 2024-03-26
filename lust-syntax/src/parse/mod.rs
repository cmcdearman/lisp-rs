use self::{
    ast::*,
    error::{ParseError, ParseResult},
};
use crate::read::sexpr::{self, AtomKind, Sexpr, SexprKind};
use lust_utils::list::List;

pub mod ast;
pub mod error;

pub fn parse(root: sexpr::Root) -> (Option<ast::Root>, Vec<ParseError>) {
    let mut items = vec![];
    let mut errs = vec![];
    for sexpr in root.sexprs {
        match parse_def(sexpr) {
            Ok(decl) => items.push(decl),
            Err(err) => errs.push(err),
        }
    }
    (Some(ast::Root {defs, span: root.span}), errs)
}

fn parse_def(sexpr: &Sexpr) -> ParseResult<Decl> {
    match sexpr.kind() {
        SexprKind::List(list) => {
            let head = list.head().ok_or(ParseError::new(
                "expected first element".to_string(),
                sexpr.span(),
            ))?;

            match head.kind() {
                SexprKind::Atom(a) => match a.kind() {
                    AtomKind::Sym(s) => match s.as_ref() {
                        "def" => {
                            let mut iter = list.iter();
                            iter.next(); // skip head
                            let name = match iter
                                .next()
                                .ok_or(ParseError::new("expected name".to_string(), head.span()))?.kind() {
                                SexprKind::Atom(a) => match a.kind() {
                                    AtomKind::Sym(s) => s.clone(),
                                    _ => {
                                        return Err(ParseError::new(
                                            "expected symbol".to_string(),
                                            head.span(),
                                        ))
                                    }
                                },
                                _ => {
                                    return Err(ParseError::new(
                                        "expected atom".to_string(),
                                        head.span(),
                                    ))
                                }
                            let expr = iter.next().ok_or(ParseError::new(
                                "expected expression".to_string(),
                                head.span(),
                            ))?;
                            Ok(Decl::new(
                                DeclKind::Def {
                                    name: name,
                                    expr: parse_expr(expr)?,
                                },
                                sexpr.span(),
                            ))
                        }
                        _ => Err(ParseError::new(
                            format!("unexpected special form: {}", s),
                            head.span(),
                        )),
                    },
                    _ => Err(ParseError::new("expected symbol".to_string(), head.span())),
                },
                _ => Err(ParseError::new("expected atom".to_string(), head.span())),
            }
                SexprKind::List(_) => todo!(),
        }
        _ => Err(ParseError::new("expected list".to_string(), sexpr.span())),
    }
        SexprKind::Atom(_) => todo!(),
}}

fn parse_expr(sexpr: &Sexpr) -> ParseResult<Expr> {
    match sexpr.kind() {
        sexpr::SexprKind::Atom(a) => match a.kind() {
            sexpr::AtomKind::Lit(l) => Ok(Expr::new(ExprKind::Lit(parse_lit(l)), sexpr.span())),
            sexpr::AtomKind::Sym(name) => {
                Ok(Expr::new(ExprKind::Ident(name.clone()), sexpr.span()))
            }
        },
        sexpr::SexprKind::List(l) => {
            let first = l.head().ok_or(ParseError::new(
                "expected first element".to_string(),
                sexpr.span(),
            ))?;
            // handle special forms
            todo!()
        }
    }
}

fn parse_pattern(sexpr: &Sexpr) -> ParseResult<Pattern> {
    match sexpr.kind() {
        SexprKind::Atom(a) => match a.kind() {
            AtomKind::Sym(s) => Ok(Pattern::new(PatternKind::Ident(s.clone()), sexpr.span())),
            AtomKind::Lit(l) => Ok(Pattern::new(PatternKind::Lit(parse_lit(l)), sexpr.span())),
        },
        SexprKind::List(list) => {
            // parse list sexprs as patterns
            list.iter().map(|s| parse_pattern(s)).collect::<ParseResult<_>>()?
        }
    }
}

fn parse_lit(lit: &sexpr::Lit) -> Lit {
    match lit.clone() {
        sexpr::Lit::Int(i) => Lit::Int(i),
        sexpr::Lit::BigInt(i) => Lit::BigInt(i),
        sexpr::Lit::Real(f) => Lit::Real(f),
        sexpr::Lit::Rational(r) => Lit::Rational(r),
        sexpr::Lit::BigRational(r) => Lit::BigRational(r),
        sexpr::Lit::String(s) => Lit::String(s),
        sexpr::Lit::Bool(b) => Lit::Bool(b),
        sexpr::Lit::Char(c) => Lit::Char(c),
    }
}
