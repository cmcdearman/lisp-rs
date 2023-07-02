use crate::{intern::InternedString, T};
use logos::{Lexer, Logos};
use num_rational::Rational64;
use std::fmt::Debug;

use self::{
    error::{Error, ReadResult},
    sexpr::{Atom, Cons, ConsList, Lit, Sexpr},
    token::{Span, Token, TokenKind},
};

pub mod error;
pub mod sexpr;
mod tests;
pub mod token;

pub struct Reader<'src> {
    src: &'src str,
    logos: Lexer<'src, TokenKind>,
    peek: Option<Token>,
    errors: Vec<Error>,
}

impl<'src> Reader<'src> {
    /// Creates a new [`Parser`].
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            logos: TokenKind::lexer(src),
            peek: None,
            errors: vec![],
        }
    }

    /// Returns the source code of the token.
    fn text(&self, token: Token) -> &'src str {
        token.lit(&self.src)
    }

    /// Returns the peek token in the stream.
    fn next(&mut self) -> Token {
        if let Some(t) = self.peek.take() {
            t
        } else {
            self.fetch_token()
        }
    }

    /// Returns the next token in the stream without consuming it.
    fn peek(&mut self) -> Token {
        if let Some(t) = self.peek {
            t
        } else {
            let t = self.fetch_token();
            self.peek = Some(t);
            t
        }
    }


    /// Returns true if the next token is of the given kind.
    fn at(&mut self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    /// Consumes the next token if it is of the given kind.
    fn eat(&mut self, expected: TokenKind) -> bool {
        if self.at(expected) {
            self.next();
            true
        } else {
            false
        }
    }

    /// Parses the source code into a [`Sexpr`].
    pub fn sexpr(&mut self) -> ReadResult<Sexpr> {
        match self.peek().kind {
            T!['('] => self.list(),
            _ => self.atom(),
        }
    }

    fn list(&mut self) -> ReadResult<Sexpr> {
        if !self.eat(T!['(']) {
            return Err(Error::new("Expected `(`"));
        }
        let mut sexprs = vec![];
        while !self.at(T![')']) {
            sexprs.push(self.sexpr()?);
        }
        if !self.eat(T![')']) {
            return Err(Error::new("Expected `)`"));
        }
        Ok(Sexpr::List(ConsList::new(
            sexprs
                .into_iter()
                .rev()
                .fold(None, |cdr, car| Some(Box::new(Cons { car, cdr }))),
        )))
    }

    fn atom(&mut self) -> ReadResult<Sexpr> {
        match self.peek().kind {
            T![int] | T![real] | T![ratio] | T![char] | T![str] | T![bool] => {
                Ok(Sexpr::Atom(Atom::Lit(self.lit()?)))
            }
            T![ident] => Ok(Sexpr::Atom(Atom::Symbol(self.symbol()?))),
            _ => Err(Error::from(format!(
                "Unexpected token in atom `{}`",
                self.peek().kind
            ))),
        }
    }

    fn lit(&mut self) -> ReadResult<Lit> {
        match self.peek().kind {
            T![int] => Ok(Lit::Int(self.int()?)),
            T![real] => Ok(Lit::Real(self.real()?)),
            T![ratio] => Ok(Lit::Rational(self.rational()?)),
            T![char] => Ok(Lit::Char(self.char()?)),
            T![str] => Ok(Lit::String(self.string()?)),
            T![bool] => Ok(Lit::Bool(self.bool()?)),
            _ => Err(Error::from(format!(
                "Unexpected token in literal `{}`",
                self.peek().kind
            ))),
        }
    }

    fn int(&mut self) -> ReadResult<i64> {
        let token = self.next();
        let text = self.text(token);
        let num = text
            .parse()
            .map_err(|_| Error::new("Failed to parse integer"))?;
        Ok(num)
    }

    fn real(&mut self) -> ReadResult<f64> {
        let token = self.next();
        let text = self.text(token);
        let num = text
            .parse()
            .map_err(|_| Error::new("Failed to parse float"))?;
        Ok(num)
    }

    fn rational(&mut self) -> ReadResult<Rational64> {
        let token = self.next();
        let text = self.text(token);
        let num = text
            .parse()
            .map_err(|_| Error::new("Failed to parse rational"))?;
        Ok(num)
    }

    fn string(&mut self) -> ReadResult<InternedString> {
        let token = self.next();
        let text = self.text(token);
        Ok(InternedString::from(&text[1..(text.len() - 1)]))
    }

    fn char(&mut self) -> ReadResult<char> {
        let token = self.next();
        let text = self.text(token);
        Ok(text
            .chars()
            .nth(1)
            .ok_or(Error::new("Failed to parse char"))?)
    }

    fn bool(&mut self) -> ReadResult<bool> {
        let token = self.next();
        let text = self.text(token);
        Ok(text == "#t")
    }

    fn symbol(&mut self) -> ReadResult<InternedString> {
        let token = self.next();
        let text = self.text(token);
        Ok(InternedString::from(text))
    }
}
