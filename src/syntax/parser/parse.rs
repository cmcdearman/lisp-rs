use super::{
    ast::{Decl, Item, Lit, Root, Symbol},
    error::ParseResult,
};
use crate::{
    syntax::{
        parser::{
            ast::Expr,
            error::{ParserError, SyntaxError},
        },
        reader::{
            sexpr::{self, Atom, Sexpr},
            token::Token,
        },
    },
    util::{node::SrcNode, span::Span},
};

pub fn parse<'src>(src: &'src str, root: &sexpr::Root) -> ParseResult<SrcNode<Root>> {
    let items = vec![];
    
    Ok(SrcNode::new(Root { items }, Span::new(0, src.len() as u32)))
}

fn parse_item<'src>(src: &'src str, sexpr: &sexpr::Sexpr) -> ParseResult<SrcNode<Item>> {
    todo!()
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
                        let cond = parse_expr(
                            src,
                            &sexpr.clone().into_iter().nth(1).ok_or(SrcNode::new(
                                SyntaxError::new("`if` expression needs cond"),
                                sexpr.span(),
                            ))?,
                        )?;
                        let then = parse_expr(
                            src,
                            &sexpr.clone().into_iter().nth(2).ok_or(SrcNode::new(
                                SyntaxError::new("`if` expression needs then"),
                                sexpr.span(),
                            ))?,
                        )?;
                        let else_ = parse_expr(
                            src,
                            &sexpr.clone().into_iter().nth(3).ok_or(SrcNode::new(
                                SyntaxError::new("`if` expression needs else"),
                                sexpr.span(),
                            ))?,
                        )?;
                        Ok(SrcNode::new(Expr::If { cond, then, else_ }, sexpr.span()))
                    }
                    "quote" => {
                        todo!()
                    }
                    "unquote" => {
                        todo!()
                    }
                    "unquote-splice" => {
                        todo!()
                    }
                    _ => {
                        todo!()
                    }
                },
                _ => todo!(),
            },
            _ => todo!(),
        },
        Sexpr::Nil => Ok(SrcNode::new(Expr::Nil, sexpr.span())),
    }
}
