use super::{
    error::{ReadResult, ReaderError, SyntaxError},
    sexpr::{Atom, Lit, Root, Sexpr},
    token::{Token, TokenKind},
};
use cstree::Syntax;
use logos::{Lexer, Logos};
use lust_util::{
    list::List,
    span::{Span, Spannable, Spanned},
};

pub struct Reader<'src> {
    src: &'src str,
    lexer: Lexer<'src, TokenKind>,
    peek: Option<Token>,
    errors: Vec<Spanned<ReaderError>>,
}

impl<'src> Reader<'src> {
    /// Creates a new [`Parser`].
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            lexer: TokenKind::lexer(src),
            peek: None,
            errors: vec![],
        }
    }

    fn fetch_token(&mut self) -> Token {
        match self.lexer.next().map(|res| (res, self.lexer.span())) {
            Some((res, s)) => match res {
                Ok(t) => t.spanned(s.into()),
                Err(_) => {
                    self.errors.push(ReaderError::LexerError.spanned(s.into()));
                    self.fetch_token()
                }
            },
            None => TokenKind::Eof.spanned(self.lexer.span().into()),
        }
    }

    fn peek(&mut self) -> Token {
        if let Some(token) = self.peek.clone() {
            token
        } else {
            let token = self.fetch_token();
            self.peek = Some(token.clone());
            token
        }
    }

    fn next(&mut self) -> Token {
        if let Some(token) = self.peek.take() {
            token
        } else {
            self.fetch_token()
        }
    }

    fn at(&mut self, kind: TokenKind) -> bool {
        self.peek().value == kind
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.next();
            true
        } else {
            let next = self.next();
            false
        }
    }

    fn text(&self) -> &'src str {
        &self.src[self.lexer.span()]
    }

    pub fn parse(&mut self) -> (Root, Vec<ReaderError>) {
        let mut sexprs = vec![];
        while !self.at(TokenKind::Eof) {
            match self.sexpr() {
                Ok(s) => sexprs.push(s),
                Err(e) => {
                    self.errors.push(e);
                    self.next();
                }
            }
            (Root { sexprs }, self.errors.clone())
        }
    }

    fn sexpr(&mut self) -> ReadResult<Spanned<Sexpr>> {
        match self.peek().0 {
            TokenKind::LParen => self.list(),
            _ => self.atom(),
        }
    }

    fn list(&mut self) -> ReadResult<Spanned<Sexpr>> {
        let start = self.peek().span;
        if !self.eat(TokenKind::LParen) {
            return Err(SyntaxError::UnmatchedParen(self.peek().span));
        }
        let mut sexprs = vec![];
        while !self.at(TokenKind::RParen) {
            let s = self.sexpr()?;
            sexprs.push(s);
            self.next();
        }
        if !self.eat(TokenKind::RParen) {
            return Err(SyntaxError::UnmatchedParen(self.peek().span).spanned(self.peek().span));
        }
        let list: List<Spanned<Sexpr>> = sexprs.into_iter().rev().collect();
        let end = self.peek().span;
        Ok(Sexpr::Cons(list).spanned(start.extend(end)))
    }

    fn atom(&mut self) -> ReadResult<Spanned<Sexpr>> {
        match self.peek().value {
            TokenKind::Int => {
                let i = self
                    .text()
                    .parse()
                    .map_err(|e| SyntaxError::LitParseError(self.peek()))?;
                let next = self.next();
                Ok(
                    Sexpr::Atom(Atom::Lit(Lit::Int(i).spanned(next.span)).spanned(next.span))
                        .spanned(next.span),
                )
            }
            TokenKind::Rational(r) => Ok((Sexpr::Atom(Atom::Lit(Lit::Rational(r))), self.peek().1)),
            TokenKind::Real(r) => Ok((Sexpr::Atom(Atom::Lit(Lit::Real(r))), self.peek().1)),
            TokenKind::Char(c) => Ok((Sexpr::Atom(Atom::Lit(Lit::Char(c))), self.peek().1)),
            TokenKind::String(s) => Ok((
                Sexpr::Atom(Atom::Lit(Lit::String(InternedString::from(
                    &s[1..(s.len() - 1)],
                )))),
                self.peek().1,
            )),
            TokenKind::Ident(name) => Ok((
                Sexpr::Atom(Atom::Symbol(InternedString::from(name))),
                self.peek().1,
            )),
            _ => Err(ReaderError::UnexpectedToken(self.peek())),
        }
    }
}
