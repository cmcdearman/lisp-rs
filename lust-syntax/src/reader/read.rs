use super::{
    error::{ReadResult, ReaderError, SyntaxError},
    sexpr::{Atom, Lit, Root, Sexpr},
    token::{Token, TokenKind},
};
use cstree::Syntax;
use logos::{Lexer, Logos};
use lust_util::{
    intern::InternedString,
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
                    while !self.at(TokenKind::LParen) {
                        self.next();
                    }
                }
            }
        }
        (Root { sexprs }, self.errors.clone())
    }

    fn sexpr(&mut self) -> ReadResult<Spanned<Sexpr>> {
        match self.peek().value {
            TokenKind::LParen => self.list(),
            _ => self.atom(),
        }
    }

    fn list(&mut self) -> ReadResult<Spanned<Sexpr>> {
        log::trace!("enter list");
        let start = self.peek().span;
        if !self.eat(TokenKind::LParen) {
            return Err(SyntaxError::UnmatchedParen(self.peek().span).spanned(self.peek().span));
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
                let i = self.text().parse().map_err(|_| {
                    SyntaxError::LitParseError(self.peek()).spanned(self.peek().span)
                })?;
                let next = self.next();
                Ok(Sexpr::Atom(Atom::Lit(Lit::Int(i)).spanned(next.span)).spanned(next.span))
            }
            TokenKind::Rational => {
                let r = self.text().parse().map_err(|e| {
                    SyntaxError::LitParseError(self.peek()).spanned(self.peek().span)
                })?;
                let next = self.next();
                Ok(Sexpr::Atom(Atom::Lit(Lit::Rational(r)).spanned(next.span)).spanned(next.span))
            }
            TokenKind::Real => {
                let r = self.text().parse().map_err(|e| {
                    SyntaxError::LitParseError(self.peek()).spanned(self.peek().span)
                })?;
                let next = self.next();
                Ok(Sexpr::Atom(Atom::Lit(Lit::Real(r)).spanned(next.span)).spanned(next.span))
            }
            TokenKind::Char => {
                let c = self.text().chars().nth(1).ok_or_else(|| {
                    SyntaxError::LitParseError(self.peek()).spanned(self.peek().span)
                })?;
                let next = self.next();
                Ok(Sexpr::Atom(Atom::Lit(Lit::Char(c)).spanned(next.span)).spanned(next.span))
            }
            TokenKind::String => {
                let s = &self.text()[1..(self.text().len() - 1)];
                let next = self.next();
                Ok(
                    Sexpr::Atom(Atom::Lit(Lit::String(InternedString::from(s))).spanned(next.span))
                        .spanned(next.span),
                )
            }
            TokenKind::Ident => {
                let s = InternedString::from(self.text());
                let next = self.next();
                Ok(Sexpr::Atom(Atom::Symbol(s).spanned(next.span)).spanned(next.span))
            }
            _ => Err(SyntaxError::UnexpectedToken(self.peek()).spanned(self.peek().span)),
        }
    }
}

mod tests {
    use super::Reader;

    #[test]
    fn read_int() {
        let src = "42";
        let mut reader = Reader::new(src);
        let (root, errs) = reader.parse();
        if !errs.is_empty() {
            panic!("{:?}", errs);
        }
        insta::assert_debug_snapshot!(root);
    }

    #[test]
    fn read_list() {
        let src = "(1 2 3)";
        let mut reader = Reader::new(src);
        let (root, errs) = reader.parse();
        if !errs.is_empty() {
            panic!("{:?}", errs);
        }
        insta::assert_debug_snapshot!(root);
    }
}
