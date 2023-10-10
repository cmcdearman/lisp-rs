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

pub fn parse(root: &sexpr::Root) -> ParseResult<SrcNode<Root>> {
    let mut decls = vec![];
    for sexpr in root.0 {
        match sexpr {
            Sexpr::Atom(_) => todo!(),
            Sexpr::Pair(_) => todo!(),
            Sexpr::Nil => todo!(),
        }
    }
    Ok(SrcNode::new(Root { decls }))
}

fn parse_decl() -> ParseResult<SrcNode<Decl>> {
    todo!()
}
