use logos::{Lexer, Logos};

use crate::T;

use sexpr::{Atom, Lit, Sexpr};

use self::{
    parser_error::{ParserError, ParserErrorKind, Result},
    sexpr::{Cons, List, Number, NIL},
    token::{Span, Token, TokenKind},
};

pub mod parser_error;
pub mod sexpr;
pub mod token;

/// Parser is a recursive descent parser for the Lust language.
pub struct Parser<'src> {
    /// The source code to parse.
    src: &'src str,

    /// The [`Lexer`] used to lex the source code.
    logos: Lexer<'src, TokenKind>,

    /// The next token to be consumed.
    peek: Option<Token>,
}

impl<'src> Parser<'src> {
    /// Creates a new [`Parser`].
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            logos: TokenKind::lexer(src),
            peek: None,
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
            self.generate()
        }
    }

    /// Returns the next token in the stream without consuming it.
    fn peek(&mut self) -> Token {
        if let Some(t) = self.peek {
            t
        } else {
            let t = self.generate();
            self.peek = Some(t);
            t
        }
    }

    /// Gets the next token from the [`Lexer`].
    fn generate(&mut self) -> Token {
        match self.logos.next().map(|t| (t, self.logos.span())) {
            None => Token {
                kind: T![EOF],
                span: Span::new(0, 0),
            },
            Some((T![;], _)) => self.generate(),
            Some((t, s)) => Token {
                kind: t,
                span: Span::from(s),
            },
        }
    }
    
    /// Returns true if the next token is of the given kind.
    fn at(&mut self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    fn consume(&mut self, expected: TokenKind) {
        let token = self.next();
        assert_eq!(
            token.kind, expected,
            "Expected to consume `{}`, but found `{}`",
            expected, token.kind
        );
    }

    pub fn parse(&mut self) -> Result<Sexpr> {
        match self.peek().kind {
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
        match self.peek().kind {
            lit @ T![int] | lit @ T![float] | lit @ T![ratio] | lit @ T![str] | lit @ T![bool] => {
                let lit_tok = self.next();
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
                let ident = self.next();
                Ok(Sexpr::Atom(Atom::Sym(self.text(ident).to_string())))
            }
            kind => {
                panic!("Unknown start of atom: `{}`", kind);
            }
        }
    }
}
