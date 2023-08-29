use super::{
    error::{ReadResult, ReaderError},
    sexpr::{Root, Sexpr},
    token::{Token, TokenKind},
};
use logos::{Lexer, Logos};
use lust_util::{
    list::List,
    span::{Span, Spannable, Spanned},
};

pub struct Reader<'src> {
    src: &'src str,
    lexer: Lexer<'src, TokenKind>,
    peek: Option<Token>,
    errors: Vec<ReaderError>,
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
                    self.errors.push(SyntaxError::LexerError.spanned(s.into()));
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

    fn eat(&mut self, kind: TokenKind) -> ReadResult<()> {
        if self.at(kind) {
            self.next();
            Ok(())
        } else {
            let next = self.next();
            Err(ReaderError::UnexpectedToken(next))
        }
    }

    fn text(&self) -> &'src str {
        &self.src[self.lexer.span()]
    }

    pub fn parse(&mut self) -> (Root, Vec<ReaderError>) {
        let mut sexprs = vec![];
        while !self.at(TokenKind::Eof) {
            if let Ok(s) = self.sexpr() {
                sexprs.push(s);
            } else {
                self.errors.push(ReaderError::UnexpectedToken(self.peek()));
                self.next();
            }
        }
        (Root { sexprs }, self.errors.clone())
    }

    fn sexpr(&mut self) -> ReadResult<Spanned<Sexpr>> {
        match self.peek().0 {
            TokenKind::LParen => self.list(),
            _ => self.atom(),
        }
    }

    fn list(&mut self) -> ReadResult<Spanned<Sexpr>> {
        let start = self.peek().span;
        if !self.eat(TokenKind::LParen)? {
            return Err(ReaderError::UnmatchedParen(self.peek().1));
        }
        let mut sexprs = vec![];
        while !self.at(&Token::RParen) {
            let s = self.sexpr()?;
            sexprs.push(s);
            self.next();
        }
        if !self.eat(&Token::RParen) {
            return Err(ReaderError::UnmatchedParen(self.peek().1));
        }
        let list: List<Spanned<Sexpr>> = sexprs.into_iter().rev().collect();
        let end = self.peek().1.end as usize;
        Ok((Sexpr::Cons(Box::new(list)), Span::from(start..end)))
    }

    fn atom(&mut self) -> ReadResult<Spanned<Sexpr>> {
        match self.peek().0 {
            Token::Int(i) => Ok((Sexpr::Atom(Atom::Lit(Lit::Int(i))), self.peek().1)),
            Token::Rational(r) => Ok((Sexpr::Atom(Atom::Lit(Lit::Rational(r))), self.peek().1)),
            Token::Real(r) => Ok((Sexpr::Atom(Atom::Lit(Lit::Real(r))), self.peek().1)),
            Token::Char(c) => Ok((Sexpr::Atom(Atom::Lit(Lit::Char(c))), self.peek().1)),
            Token::String(s) => Ok((
                Sexpr::Atom(Atom::Lit(Lit::String(InternedString::from(
                    &s[1..(s.len() - 1)],
                )))),
                self.peek().1,
            )),
            Token::Ident(name) => Ok((
                Sexpr::Atom(Atom::Symbol(InternedString::from(name))),
                self.peek().1,
            )),
            _ => Err(ReaderError::UnexpectedToken(self.peek())),
        }
    }
}
