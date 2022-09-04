use std::iter::Peekable;

use crate::ast::{Atom, Lit, Sexpr};
use crate::token::{TokenKind, TokenStream};

pub fn parse(tokens: &mut Peekable<TokenStream>, alloc: &mut impl FnMut(Sexpr) -> u32) -> u32 {
    match tokens.peek().unwrap().kind {
        TokenKind::LParen => { cons(tokens, alloc) }
        _ => alloc(atom(tokens)),
    }
}

fn cons(tokens: &mut Peekable<TokenStream>, alloc: &mut impl FnMut(Sexpr) -> u32) -> u32 {
    tokens.next();
    let mut last = alloc(Sexpr::Nil);
    while tokens.peek().unwrap().kind != TokenKind::RParen {
        let next = parse(tokens, alloc);
        last = alloc(Sexpr::Cons(last, next));
    };
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
        | TokenKind::Mod
        | TokenKind::Let
        | TokenKind::Lambda => Sexpr::Atom(Atom::Sym(tokens.next().unwrap().lit.to_string())),
        kind => {
            panic!("Unknown start of atom: `{}`", kind);
        }
    }
}

// let mut new_tail = elements.len();
// elements.push(SExpr::Cons(0, 0));
// match &mut elements[tail] {
//     Atom(_) => unreachable!(), // Sadly, this is neccessary
//     Cons(_, tail) => *tail = new_tail,
// }
// tail = new_tail;
