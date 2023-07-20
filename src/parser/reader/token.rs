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

use super::error::ReaderError;

#[derive(Logos, Debug, Clone, Default, PartialEq)]
pub enum Token {
    #[default]
    Eof,
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,
    #[regex(r#";[^\n]*"#)]
    Comment,
    #[regex(r#"[^\[\]()\s,{};]+"#, |lex| InternedString::from(lex.slice()))]
    Ident(InternedString),
    #[regex(r#"([1-9]\d*|0)"#, priority = 3, callback = |lex| BigInt::parse_bytes(lex.slice().as_bytes(), 10))]
    Int(BigInt),
    #[regex(r#"(\+|-)?\d+/\d+"#, |lex| lex.slice().parse().ok())]
    Rational(BigRational),
    #[regex(r#"((\d+(\.\d+))|(\.\d+))([Ee](\+|-)?\d+)?"#, priority = 2, callback = |lex| lex.slice().parse().ok())]
    Real(f64),
    #[regex(r#"'\w'"#, |lex| lex.slice().chars().nth(1))]
    Char(char),
    #[regex(r#""((\\"|\\\\)|[^\\"])*""#, |lex| InternedString::from(lex.slice()))]
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
}

impl TokenStream {
    pub fn new<'src>(src: &'src str) -> Result<Self, Vec<Spanned<ReaderError>>> {
        let (tokens, errors): (
            Vec<Option<Spanned<Token>>>,
            Vec<Option<Spanned<ReaderError>>>,
        ) = Token::lexer(src)
            .spanned()
            .map(|(res, span)| match res {
                Ok(t) => (Some((t, Span::from(span))), None),
                Err(err) => (None, Some((ReaderError::LexerError, Span::from(span)))),
            })
            .unzip();

        if errors.iter().any(|e| e.is_some()) {
            Err(errors.into_iter().flatten().collect())
        } else {
            Ok(Self {
                tokens: tokens
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .peekable(),
            })
        }
    }

    pub fn peek(&mut self) -> Spanned<Token> {
        self.tokens
            .peek()
            .unwrap_or(&(Token::Eof, Span::new(0, 0)))
            .clone()
    }

    pub fn next(&mut self) -> Spanned<Token> {
        self.tokens.next().unwrap_or((Token::Eof, Span::new(0, 0)))
    }

    pub fn at(&mut self, token: &Token) -> bool {
        self.peek().0 == token.clone()
    }

    pub fn eat(&mut self, token: &Token) -> bool {
        if self.at(token) {
            self.next();
            true
        } else {
            false
        }
    }
}
