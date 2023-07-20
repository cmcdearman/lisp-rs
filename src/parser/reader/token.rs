use super::error::Error;
use crate::{
    intern::InternedString,
    span::{Span, Spanned},
};
use logos::Logos;
use num_bigint::BigInt;
use num_rational::BigRational;
use std::{
    fmt::{Debug, Display},
    iter::Peekable,
    vec::IntoIter,
};

#[derive(Logos, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum Token {
    #[default]
    Eof,
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,
    #[regex(r#";[^\n]*"#)]
    Comment,
    #[regex(r#"[^\[\]()\s,{};]+"#)]
    Ident(InternedString),
    #[regex(r#"([1-9]\d*|0)"#, priority = 3)]
    Int(BigInt),
    #[regex(r#"(\+|-)?\d+/\d+"#)]
    Rational(BigRational),
    #[regex(r#"((\d+(\.\d+))|(\.\d+))([Ee](\+|-)?\d+)?"#, priority = 2)]
    Real(f64),
    #[regex(r#"'\w'"#)]
    Char(char),
    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    String(InternedString),

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBrack,
    #[token("]")]
    RBrack,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(":")]
    Colon,
    #[token(".")]
    Period,
    #[token(",")]
    Comma,
    #[token("#")]
    Hash,
    #[token("'")]
    Quote,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Eof => "<EOF>",
                Token::Whitespace => "Whitespace",
                Token::Comment => "Comment",
                Token::Ident(name) => &format!("Ident({})", name),
                Token::Int(i) => &format!("Int({})", i),
                Token::Rational(r) => &format!("Rational({})", r),
                Token::Real(r) => &format!("Real({})", r),
                Token::Char(c) => &format!("Char({})", c),
                Token::String(s) => &format!("String({})", s),
                Token::LParen => "(",
                Token::RParen => ")",
                Token::LBrack => "[",
                Token::RBrack => "]",
                Token::LBrace => "{",
                Token::RBrace => "}",
                Token::Colon => ":",
                Token::Period => ".",
                Token::Comma => ",",
                Token::Hash => "#",
                Token::Quote => "'",
            }
        )
    }
}

pub struct TokenStream {
    tokens: Peekable<IntoIter<Spanned<Token>>>,
    errors: Vec<Error>,
}

impl TokenStream {
    pub fn new<'src>(src: &'src str) -> Self {
        // let tokens: Vec<Spanned<Token>> = Token::lexer(src)
        //     .spanned()
        //     .filter_map(|(t, s)| t.ok().map(|t| (t, s.into())))
        //     .collect();
        // Self {
        //     tokens: tokens.into_iter().peekable(),
        // }
        let (tokens, errors): (Vec<Option<Spanned<Token>>>, Vec<Option<Error>>) = TokenKind::lexer(src)
        .spanned()
        .map(|(kind, span)| match kind {
            Ok(kind) => (
                Some(Token {
                    kind: kind,
                    span: span.into(),
                }),
                None,
            ),
            Err(err) => (
                None,
                Some(Error(format!("Lexer error - {}", Span::from(span)))),
            ),
        })
        .unzip();

    Self {
        tokens: tokens
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .into_iter()
            .peekable(),
        errors: errors.into_iter().flatten().collect(),
    }
    }

    /// Fetches the next token from the [`Lexer`].
    fn fetch_token(&mut self) -> Spanned<Token> {
        match self.tokens.next() {
            None => (Token::Eof, Span::from(0..0)),
            Some((t, s)) => match t {
                Token::Comment => self.fetch_token(),
                t => (t, s),
                Err(e) => {
                    self.errors
                        .push(Error::from(format!("Lexer error: {:?}", e)));
                    // Token {
                    //     kind: T![EOF],
                    //     span: Span::new(s.start as u32, s.end as u32),
                    // }
                    Token::Eof
                }
            },
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub fn at(&mut self, kind: Token) -> bool {
        self.peek().map(|t| t.kind == kind).unwrap_or(false)
    }

    pub fn eat(&mut self, kind: Token) -> bool {
        if self.at(kind) {
            self.next();
            true
        } else {
            false
        }
    }
}
