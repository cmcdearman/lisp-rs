use std::iter::Peekable;

use crate::{token::{TokenKind, TokenStream}, ast::{object::{Object, Lit, Atom}, symbol::Symbol, number::Number, list::List}};

pub fn parse(tokens: &mut Peekable<TokenStream>) -> Object {
    match tokens.peek().unwrap().kind {
        TokenKind::LParen => {
            tokens.next();
            list(tokens)
        }
        _ => atom(tokens),
    }
}

fn list(tokens: &mut Peekable<TokenStream>) -> Object {
    let mut head = List::NIL;
    let mut tail = &mut list;

    while tokens.peek().unwrap().kind != TokenKind::RParen {
        todo!()
    }
    tokens.next();

    Object::List(head)
}

fn atom(tokens: &mut Peekable<TokenStream>) -> Object {
    match tokens.peek().unwrap().kind {
        lit @ TokenKind::Num | lit @ TokenKind::String | lit @ TokenKind::Bool => {
            let lit_text = tokens.next().unwrap().lit;
            let lit = match lit {
                TokenKind::Num => Lit::Num(
                    Number(
                    lit_text
                        .parse()
                        .expect(&format!("invalid floating point literal: `{}`", lit_text)),
                )),
                TokenKind::String => Lit::Str(lit_text[1..(lit_text.len() - 1)].to_string()),
                TokenKind::Bool => Lit::Bool(lit_text.parse().unwrap()),
                _ => unreachable!(),
            };
            Object::Atom(Atom::Lit(lit))
        }
        TokenKind::Ident
        | TokenKind::Add
        | TokenKind::Sub
        | TokenKind::Mul
        | TokenKind::Quo
        | TokenKind::Let
        | TokenKind::Fn
        | TokenKind::Mod => Object::Atom(Atom::Sym(Symbol::from(&*tokens.next().unwrap().lit))),
        kind => {
            panic!("Unknown start of atom: `{}`", kind);
        }
    }
}
