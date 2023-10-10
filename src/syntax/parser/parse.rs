use super::ast::{Decl, Lit, Root, Symbol};
use crate::{
    syntax::{
        parser::ast::Expr,
        reader::{
            sexpr::{Atom, Sexpr},
            token::Token,
        },
    },
    util::{node::SrcNode, span::Span},
};
use chumsky::{extra, input::ValueInput, prelude::*, recursive::recursive, select, Parser};
pub type ParseError<'a> = Rich<'a, Token, Span>;

fn lit_parser<'a, I: ValueInput<'a, Token = Atom, Span = Span>>(
) -> impl Parser<'a, I, Lit, extra::Err<Rich<'a, Atom, Span>>> {
    select! {
        Atom::Number(n) => Lit::Number(n),
        Atom::String(s) => Lit::String(s),
    }
}

fn symbol_parser<'a, I: ValueInput<'a, Token = Atom, Span = Span>>(
) -> impl Parser<'a, I, Symbol, extra::Err<Rich<'a, Atom, Span>>> {
    select! {
        Atom::Symbol(s) => Symbol(s),
    }
}

fn expr_parser<'a, I: ValueInput<'a, Token = Sexpr, Span = Span>>(
) -> impl Parser<'a, I, SrcNode<Expr>, extra::Err<Rich<'a, Sexpr, Span>>> {
    let ident = symbol_parser().map(Expr::Symbol);
    let lit = lit_parser().map(Expr::Lit);

    recursive(|expr| {
        let list = expr
            .nested_in(select_ref! {
                Sexpr::Pair(pair) => pair.span(),
            })
            .map(|pair| {});
        ident.or(lit)
    })
}

// pub fn parse(root: &sexpr::Root) -> ParseResult<SrcNode<Root>> {
//     let mut decls = vec![];
//     for sexpr in root.iter() {
//         match sexpr {
//             sexpr::Sexpr::Atom(_) => todo!(),
//             sexpr::Sexpr::Cons(_) => todo!(),
//         }
//     }
//     Ok(SrcNode::new(Root { decls }))
// }

// fn parse_decl() -> ParseResult<SrcNode<Decl>> {
//     todo!()
// }
