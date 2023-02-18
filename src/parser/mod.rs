use std::{cell::RefCell, iter::Peekable, rc::Rc, vec::IntoIter};

use either::Either;

use crate::T;

use sexpr::{Atom, Lit, Sexpr};

use self::{
    error::{ParserError, Result},
    lexer::Lexer,
    sexpr::Number,
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

    pub fn parse(&mut self) -> Result<Sexpr> {
        match self.peek() {
            T!['('] => {
                self.consume(T!['(']);
                self.cons()
            }
            _ => self.atom(),
        }
    }

    fn cons(&mut self) -> Result<Sexpr> {
        if self.at(T![')']) {
            return Ok(Sexpr::Nil);
        }

        let car = self.parse()?;
        let mut cdr = Sexpr::Nil;
        let mut rest = vec![];

        while !self.at(T![')']) {
            rest.push(self.parse()?)
        }

        for sexpr in rest.into_iter().rev() {
            cdr = Sexpr::Cons(Box::new(sexpr), Box::new(cdr));
        }

        Ok(Sexpr::Cons(Box::new(car), Box::new(cdr)))
    }

    fn atom(&mut self) -> Result<Sexpr> {
        match self.peek() {
            lit @ T![int] | lit @ T![float] | lit @ T![str] | lit @ T![bool] => {
                let lit_text = {
                    let lit_tok = self.next().expect("expected token but found None");
                    self.text(lit_tok)
                };
                let lit = match lit {
                    T![int] => Lit::Number(Number::Fixnum(
                        lit_text
                            .parse()
                            .map_err(|_| ParserError::new("invalid integer literal"))?,
                    )),
                    T![float] => Lit::Number(Number::Float(
                        lit_text
                            .parse()
                            .map_err(|_| ParserError::new("invalid floating point literal"))?,
                    )),
                    T![str] => Lit::Str(lit_text[1..(lit_text.len() - 1)].to_string()),
                    T![bool] => Lit::Bool(lit_text.parse().expect("invalid bool literal")),
                    _ => unreachable!(),
                };
                Ok(Sexpr::Atom(Atom::Lit(lit)))
            }
            TokenKind::Ident => {
                let ident = self.next().ok_or(ParserError::new(""))?;
                Ok(Sexpr::Atom(Atom::Sym(self.text(ident).to_string())))
            }
            kind => {
                panic!("Unknown start of atom: `{}`", kind);
            }
        }
    }
}
