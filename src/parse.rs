use std::{borrow::BorrowMut, cell::RefCell, iter::Peekable, rc::Rc};

use crate::{
    ast::{
        cons::Cons,
        list::List,
        number::{FixNum, Number},
        object::{Atom, Lit, Object},
        symbol::Symbol,
    },
    token::{Token, TokenKind, TokenStream},
};

pub fn parse(tokens: &mut Peekable<TokenStream>) -> Result<Object, String> {
    match tokens.peek().unwrap().kind {
        TokenKind::LParen => {
            tokens.next();
            list(tokens)
        }
        _ => atom(tokens),
    }
}

fn list(tokens: &mut Peekable<TokenStream>) -> Result<Object, String> {
    let mut new_list = List { head: None };
    let mut tail: Option<Rc<RefCell<Cons>>> = None;

    while tokens.peek().unwrap().kind != TokenKind::RParen {
        let new_cons = Rc::new(RefCell::new(Cons {
            car: parse(tokens)?,
            cdr: None,
        }));
        if new_list.head.is_none() {
            new_list.head = Some(new_cons.clone());
        } else if let Some(tail_cons) = tail {
            tail_cons.as_ref().borrow_mut().cdr = Some(new_cons.clone());
        }

        tail = Some(new_cons);
    }

    Ok(Object::List(new_list))
}

fn atom(tokens: &mut Peekable<TokenStream>) -> Result<Object, String> {
    match tokens.peek().unwrap().kind {
        lit @ TokenKind::Int
        | lit @ TokenKind::Float
        | lit @ TokenKind::String
        | lit @ TokenKind::Bool => {
            let lit_text = tokens.next().unwrap().lit;
            let lit = match lit {
                TokenKind::Int => {
                    Lit::Num(Number::FixNum(FixNum::Integer(lit_text.parse().expect(
                        &format!("invalid floating point literal: `{}`", lit_text),
                    ))))
                }
                TokenKind::Float => {
                    Lit::Num(Number::FixNum(FixNum::Float(lit_text.parse().expect(
                        &format!("invalid floating point literal: `{}`", lit_text),
                    ))))
                }
                TokenKind::String => Lit::Str(lit_text[1..(lit_text.len() - 1)].to_string()),
                TokenKind::Bool => Lit::Bool(lit_text.parse().unwrap()),
                _ => unreachable!(),
            };
            Ok(Object::Atom(Atom::Lit(lit)))
        }
        TokenKind::Ident => Ok(Object::Atom(Atom::Sym(Symbol::from(
            &*tokens.next().ok_or("end of tokens".to_string())?.lit
        )))),
        kind => {
            panic!("Unknown start of atom: `{}`", kind);
        }
    }
}
