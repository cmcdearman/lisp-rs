use std::iter::Peekable;

use crate::{
    ast::{
        list::List,
        number::{FixNum, Number},
        object::{Atom, Lit, Object},
        symbol::Symbol,
    },
    token::{Token, TokenKind, TokenStream},
};

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
    let mut tail = &mut head;
    let mut slot;

    while tokens.peek().unwrap().kind != TokenKind::RParen {
        let car = parse(tokens);
        *tail = tail.cons(car);
        slot = tail.cdr();
        tail = &mut slot;
    }
    tokens.next();
    Object::List(head)
}

fn atom(tokens: &mut Peekable<TokenStream>) -> Object {
    match tokens.peek().unwrap().kind {
        lit @ TokenKind::Int
        | lit @ TokenKind::Float
        | lit @ TokenKind::String
        | lit @ TokenKind::Bool => {
            let lit_text = tokens.next().unwrap().lit;
            let lit = match lit {
                TokenKind::Int => Lit::Num(Number::FixNum(FixNum::Integer(
                    lit_text
                        .parse()
                        .expect(&format!("invalid floating point literal: `{}`", lit_text)),
                ))),
                TokenKind::Float => Lit::Num(Number::FixNum(FixNum::Float(
                    lit_text
                        .parse()
                        .expect(&format!("invalid floating point literal: `{}`", lit_text)),
                ))),
                TokenKind::String => Lit::Str(lit_text[1..(lit_text.len() - 1)].to_string()),
                TokenKind::Bool => Lit::Bool(lit_text.parse().unwrap()),
                _ => unreachable!(),
            };
            Object::Atom(Atom::Lit(lit))
        }
        TokenKind::Ident => Object::Atom(Atom::Sym(Symbol::from(&*tokens.next().unwrap().lit))),
        kind => {
            panic!("Unknown start of atom: `{}`", kind);
        }
    }
}
