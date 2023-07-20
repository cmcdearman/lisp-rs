use crate::intern::InternedString;
use num_rational::Rational64;

use self::{
    error::{Error, ReadResult},
    sexpr::{Atom, Cons, ConsList, Lit, Sexpr},
    token::{Token, TokenStream},
};

pub mod error;
pub mod sexpr;
mod tests;
pub mod token;

pub struct Reader {
    tokens: TokenStream,
    errors: Vec<Error>,
}

impl Reader {
    /// Creates a new [`Parser`].
    pub fn new(tokens: TokenStream) -> Self {
        Self {
            tokens,
            errors: vec![],
        }
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
            return Err(Error::new("Expected `(`"));
        }
        let mut sexprs = vec![];
        while !self.tokens.at(&Token::RParen) {
            sexprs.push(self.sexpr()?);
        }
        if !self.tokens.eat(&Token::RParen) {
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
        match self.tokens.peek().0 {
            Token::Int(_)
            | Token::Rational(_)
            | Token::Real(_)
            | Token::Char(_)
            | Token::String(_) => Ok(Sexpr::Atom(Atom::Lit(self.lit()?))),
            Token::Ident(_) => Ok(Sexpr::Atom(Atom::Symbol(self.symbol()?))),
            _ => Err(Error::from(format!(
                "Unexpected token in atom `{}`",
                self.tokens.peek().0
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

    fn symbol(&mut self) -> ReadResult<InternedString> {
        let token = self.tokens.next().0;
        match token {
            Token::Ident(text) => Ok(InternedString::from(text)),
            _ => Err(Error::from(format!(
                "Unexpected token in symbol `{}`",
                token
            ))),
        }
    }
}
