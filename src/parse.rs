use std::iter::Peekable;

use crate::ast::{Atom, Lit, Sexpr};
use crate::token::{TokenKind, TokenStream};

pub fn parse(tokens: &mut Peekable<TokenStream>) -> Sexpr {
    match tokens.peek().unwrap().kind {
        TokenKind::LParen => {
            tokens.next();
            list(tokens)
        }
        _ => atom(tokens),
    }
}

fn list(tokens: &mut Peekable<TokenStream>) -> Sexpr {
    let mut list = Vec::new();

    while tokens.peek().unwrap().kind != TokenKind::RParen {
        tokens.next(); 
        list.push(parse(tokens));
    }
    println!("{}", tokens.peek().unwrap());
    tokens.next();

    Sexpr::List(list)
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
