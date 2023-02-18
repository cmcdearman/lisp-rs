use std::{cell::RefCell, iter::Peekable, rc::Rc, vec::IntoIter};

use either::Either;

use crate::T;

use sexpr::{Atom, Lit, Sexpr};

use self::{
    error::{ParserError, Result},
    lexer::Lexer,
    token::{Token, TokenKind},
};

pub mod error;
pub mod lexer;
pub mod sexpr;
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

    pub fn sexpr(&mut self) -> Result<Sexpr> {
        match self.peek() {
            T!['('] => {
                self.consume(T!['(']);
                self.list()
            }
            _ => self.atom(),
        }
    }

    fn list(&mut self) -> Result<Sexpr> {
        let mut new_list = List { head: None };
        let mut tail: Option<Rc<RefCell<Cons>>> = None;

        while !self.at(T![')']) {
            let new_cons = Rc::new(RefCell::new(Cons {
                car: self.sexpr()?,
                cdr: None,
            }));
            if new_list.head.is_none() {
                new_list.head = Some(new_cons.clone());
            } else if let Some(tail_cons) = tail {
                tail_cons.as_ref().borrow_mut().cdr = Some(new_cons.clone());
            }

            tail = Some(new_cons);
        }

        Ok(Sexpr::List(new_list))
    }

    fn atom(&mut self) -> Result<Sexpr> {
        match self.peek() {
            lit @ T![int] | lit @ T![float] | lit @ T![str] | lit @ T![bool] => {
                let lit_text = {
                    let lit_tok = self.next().expect("expected token but found None");
                    self.text(lit_tok)
                };
                let lit = match lit {
                    T![int] => Lit::Number(Number::Integer(
                        lit_text.parse::<Integer>().map_err(|e| ParserError(e.0))?,
                    )),
                    T![float] => Lit::Number(Number::Float(
                        lit_text.parse::<Float>().map_err(|e| ParserError(e.0))?,
                    )),
                    T![str] => Lit::Str(lit_text[1..(lit_text.len() - 1)].to_string()),
                    T![bool] => Lit::Bool(lit_text.parse().expect("invalid bool literal")),
                    _ => unreachable!(),
                };
                Ok(Sexpr::Atom(Atom::Lit(lit)))
            }
            TokenKind::Ident => {
                let ident = self.next().ok_or(ParserError::new("t".to_string()))?;
                Ok(Sexpr::Atom(Atom::Sym(Symbol::from(self.text(ident)))))
            }
            kind => {
                panic!("Unknown start of atom: `{}`", kind);
            }
        }
    }
}
