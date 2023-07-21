use self::{
    error::{ReadResult, ReaderError},
    sexpr::{Atom, Lit, Sexpr},
    token::{Token, TokenStream},
};
use crate::{intern::InternedString, list::List};

pub mod error;
pub mod sexpr;
mod tests;
pub mod token;

pub struct Reader {
    tokens: TokenStream,
}

impl Reader {
    /// Creates a new [`Parser`].
    pub fn new(tokens: TokenStream) -> Self {
        Self { tokens }
    }

    /// Parses the source code into a [`Sexpr`].
    pub fn sexpr(&mut self) -> ReadResult<Sexpr> {
        match self.tokens.peek().0 {
            Token::LParen => self.list(),
            _ => self.atom(),
        }
    }

    fn list(&mut self) -> ReadResult<Sexpr> {
        if !self.tokens.eat(&Token::LParen) {
            return Err(ReaderError::UnmatchedParen(self.tokens.peek().1));
        }
        let mut sexprs = vec![];
        while !self.tokens.at(&Token::RParen) {
            let s = self.sexpr()?;
            sexprs.push(s);
            self.tokens.next();
        }
        if !self.tokens.eat(&Token::RParen) {
            return Err(ReaderError::UnmatchedParen(self.tokens.peek().1));
        }
        let list: List<Sexpr> = sexprs.into_iter().rev().collect();
        Ok(Sexpr::Cons(Box::new(list)))
    }

    fn atom(&mut self) -> ReadResult<Sexpr> {
        match self.tokens.peek().0 {
            Token::Int(_)
            | Token::Rational(_)
            | Token::Real(_)
            | Token::Char(_)
            | Token::String(_) => Ok(Sexpr::Atom(Atom::Lit(self.lit()?))),
            Token::Ident(name) => Ok(Sexpr::Atom(Atom::Symbol(InternedString::from(name)))),
            _ => Err(ReaderError::UnexpectedToken(self.tokens.peek())),
        }
    }

    fn lit(&mut self) -> ReadResult<Lit> {
        match self.tokens.peek().0 {
            Token::Int(i) => Ok(Lit::Int(i)),
            Token::Rational(r) => Ok(Lit::Rational(r)),
            Token::Real(r) => Ok(Lit::Real(r)),
            Token::Char(c) => Ok(Lit::Char(c)),
            Token::String(s) => Ok(Lit::String(InternedString::from(&s[1..(s.len() - 1)]))),
            _ => Err(ReaderError::UnexpectedToken(self.tokens.next())),
        }
    }
}
