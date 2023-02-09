use std::{cell::RefCell, iter::Peekable, rc::Rc};

use crate::{
    object::{
        cons::Cons,
        list::List,
        number::{
            integer::{fixnum::FixNum, Integer},
            Number,
        },
        symbol::Symbol,
        Atom, Lit, Object,
    },
    token::{TokenKind, TokenStream},
};

pub mod error;
pub mod lexer;
pub mod token;

pub struct Parser<'src> {
    src: &'src str,
    tokens: Peekable<Either<Lexer<'src>, IntoIter<Token>>>,
}

impl<'src> Parser<'src> {
    pub fn new(src: &'src str, lazy: bool) -> Self {
        Self {
            src,
            tokens: if lazy {
                Either::Left(Lexer::new(src, true)).peekable()
            } else {
                Either::Right(Lexer::new(src, true).collect::<Vec<Token>>().into_iter()).peekable()
            },
        }
    }

    fn text(&self, token: Token) -> &'src str {
        token.lit(&self.src)
    }

    fn peek(&mut self) -> TokenKind {
        self.tokens
            .peek()
            .map(|token| token.kind)
            .unwrap_or(T![EOF])
    }

    fn at(&mut self, kind: TokenKind) -> bool {
        self.peek() == kind
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn consume(&mut self, expected: TokenKind) {
        let token = self.next().expect(&format!(
            "Expected to consume `{}`, but there was no next token",
            expected
        ));
        assert_eq!(
            token.kind, expected,
            "Expected to consume `{}`, but found `{}`",
            expected, token.kind
        );
    }

    pub fn sexpr(tokens: &mut Peekable<TokenStream>) -> Result<Object, String> {
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
                    TokenKind::Int => Lit::Num(Number::Integer(Integer::FixNum(FixNum(
                        lit_text
                            .parse()
                            .expect(&format!("invalid floating point literal: `{}`", lit_text)),
                    )))),
                    TokenKind::Float => Lit::Num(Number::Float(
                        lit_text
                            .parse()
                            .expect(&format!("invalid floating point literal: `{}`", lit_text)),
                    )),
                    TokenKind::String => Lit::Str(lit_text[1..(lit_text.len() - 1)].to_string()),
                    TokenKind::Bool => Lit::Bool(lit_text.parse().unwrap()),
                    _ => unreachable!(),
                };
                Ok(Object::Atom(Atom::Lit(lit)))
            }
            TokenKind::Ident => Ok(Object::Atom(Atom::Sym(Symbol::from(
                &*tokens.next().ok_or("end of tokens".to_string())?.lit,
            )))),
            kind => {
                panic!("Unknown start of atom: `{}`", kind);
            }
        }
    }
}
