use std::iter::Peekable;

use crate::ast::{Atom, Lit, Sexpr};
use crate::token::{TokenKind, TokenStream};

pub fn parse(tokens: &mut Peekable<TokenStream>, alloc: &mut impl FnMut(Sexpr) -> u32) -> u32 {
    match tokens.peek().unwrap().kind {
        TokenKind::LParen => cons(tokens, alloc),
        _ => alloc(atom(tokens)),
    }
}

fn cons(tokens: &mut Peekable<TokenStream>, alloc: &mut impl FnMut(Sexpr) -> u32) -> u32 {
    tokens.next();
    let mut items = Vec::new();
    while tokens.peek().unwrap().kind != TokenKind::RParen {
        items.push(parse(tokens, alloc));
    }
    tokens.next();
    let mut last = alloc(Sexpr::Nil);
    for item in items.into_iter().rev() {
        last = alloc(Sexpr::Cons(item, last));
    }
    last
}

fn atom(tokens: &mut Peekable<TokenStream>) -> Sexpr {
    match tokens.peek().unwrap().kind {
        lit @ TokenKind::Num | lit @ TokenKind::String | lit @ TokenKind::Bool => {
            let lit_text = tokens.next().unwrap().lit;
            let lit = match lit {
                TokenKind::Num => Lit::Num(
                    lit_text
                        .parse()
                        .expect(&format!("invalid floating point literal: `{}`", lit_text)),
                ),
                TokenKind::String => Lit::Str(lit_text[1..(lit_text.len() - 1)].to_string()),
                TokenKind::Bool => Lit::Bool(lit_text.parse().unwrap()),
                _ => unreachable!(),
            };
            Sexpr::Atom(Atom::Lit(lit))
        }
        TokenKind::Ident
        | TokenKind::Add
        | TokenKind::Sub
        | TokenKind::Mul
        | TokenKind::Quo
        | TokenKind::Let
        | TokenKind::Lambda
        | TokenKind::Mod => Sexpr::Atom(Atom::Sym(tokens.next().unwrap().lit.to_string())),
        kind => {
            panic!("Unknown start of atom: `{}`", kind);
        }
    }
}
