use std::{iter::Peekable, vec::IntoIter};

use either::Either;

use crate::T;

use sexpr::{Atom, Lit, Sexpr};

use self::{
    lexer::{
        token::{Span, Token, TokenKind},
        Lexer,
    },
    parser_error::{ParserError, ParserErrorKind, Result},
    sexpr::{Cons, List, Number, NIL},
};

pub mod lexer;
pub mod parser_error;
pub mod sexpr;

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
                self.list()
            }
            _ => self.atom(),
        }
    }

    fn list(&mut self) -> Result<Sexpr> {
        if self.at(T![')']) {
            return Ok(NIL);
        }

        let car = Box::new(self.parse()?);
        let mut cdr = None;
        let mut rest = vec![];

        while !self.at(T![')']) {
            rest.push(self.parse()?)
        }

        self.consume(T![')']);

        for sexpr in rest.into_iter().rev() {
            cdr = Some(Box::new(Cons { car: sexpr, cdr }));
        }

        Ok(Sexpr::List(List::new(Some(Box::new(Cons {
            car: *car,
            cdr,
        })))))
    }

    fn atom(&mut self) -> Result<Sexpr> {
        match self.peek() {
            lit @ T![int] | lit @ T![float] | lit @ T![ratio] | lit @ T![str] | lit @ T![bool] => {
                let lit_tok = self
                    .next()
                    .expect("expected `Token` but found `Option` None");
                let lit_text = self.text(lit_tok);
                let lit = match lit {
                    T![int] => Lit::Number(Number::Fixnum(lit_text.parse().map_err(|_| {
                        ParserError::new(ParserErrorKind::ParseIntegerError, lit_tok.span)
                    })?)),
                    T![float] => Lit::Number(Number::Float(lit_text.parse().map_err(|_| {
                        ParserError::new(ParserErrorKind::ParseFloatError, lit_tok.span)
                    })?)),
                    T![ratio] => Lit::Number(Number::Rational(lit_text.parse().map_err(|_| {
                        ParserError::new(ParserErrorKind::ParseRationalError, lit_tok.span)
                    })?)),
                    T![str] => Lit::Str(lit_text[1..(lit_text.len() - 1)].to_string()),
                    T![bool] => Lit::Bool(lit_text.parse().expect("invalid bool literal")),
                    _ => unreachable!(),
                };
                Ok(Sexpr::Atom(Atom::Lit(lit)))
            }
            TokenKind::Ident => {
                let ident = self
                    .next()
                    .expect("expected `Token` but found `Option` None");
                Ok(Sexpr::Atom(Atom::Sym(self.text(ident).to_string())))
            }
            kind => {
                panic!("Unknown start of atom: `{}`", kind);
            }
        }
    }
}
