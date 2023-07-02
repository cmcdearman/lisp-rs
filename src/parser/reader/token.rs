use std::{
    fmt::{Debug, Display},
    iter::Peekable,
    ops::{Index, Range},
    vec::IntoIter,
};

use logos::Logos;
use serde_json::map::IntoIter;

#[derive(Logos, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum TokenKind {
    #[default]
    Eof,
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,
    #[regex(r#";[^\n]*"#)]
    Comment,
    #[regex(r#"[^\[\]()\s,{};]+"#)]
    Ident,
    #[regex(r#"([1-9]\d*|0)"#, priority = 3)]
    Int,
    #[regex(r#"(\+|-)?\d+/\d+"#)]
    Rational,
    #[regex(r#"((\d+(\.\d+))|(\.\d+))([Ee](\+|-)?\d+)?"#, priority = 2)]
    Real,
    #[regex(r#"'\w'"#)]
    Char,
    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    String,
    #[regex(r#"(true)|(false)"#)]
    Bool,

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

#[macro_export]
macro_rules! T {
    [EOF] => {
        TokenKind::Eof
    };
    [ws] => {
        TokenKind::Whitespace
    };
    [;] => {
        TokenKind::Comment
    };
    [ident] => {
        TokenKind::Ident
    };
    [int] => {
        TokenKind::Int
    };
    [real] => {
        TokenKind::Real
    };
    [ratio] => {
        TokenKind::Rational
    };
    [char] => {
        TokenKind::Char
    };
    [str] => {
        TokenKind::String
    };
    [bool] => {
        TokenKind::Bool
    };
    ['('] => {
        TokenKind::LParen
    };
    [')'] => {
        TokenKind::RParen
    };
    ['['] => {
        TokenKind::LBrack
    };
    [']'] => {
        TokenKind::RBrack
    };
    ['{'] => {
        TokenKind::LBrace
    };
    ['}'] => {
        TokenKind::RBrace
    };
    [:] => {
        TokenKind::Colon
    };
    [.] => {
        TokenKind::Period
    };
    [,] => {
        TokenKind::Comma
    };
    [#] => {
        TokenKind::Hash
    };
    [quote] => {
        TokenKind::Quote
    };
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                T![EOF] => "<EOF>",
                T![ws] => "Whitespace",
                T![;] => "Comment",
                T![ident] => "Ident",
                T![int] => "Int",
                T![real] => "Real",
                T![ratio] => "Rational",
                T![char] => "Char",
                T![str] => "String",
                T![bool] => "Bool",
                T!['('] => "(",
                T![')'] => ")",
                T!['['] => "[",
                T![']'] => "]",
                T!['{'] => "{",
                T!['}'] => "}",
                T![:] => ":",
                T![.] => ".",
                T![,] => ",",
                T![#] => "#",
                T![quote] => "'",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.start, self.end)
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.start as usize..span.end as usize
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start as u32,
            end: range.end as u32,
        }
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[Range::from(index)]
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn len(&self) -> usize {
        (self.span.end - self.span.start) as usize
    }

    pub fn lit<'a>(&self, src: &'a str) -> &'a str {
        &src[self.span]
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {}", self.kind, self.span)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

pub struct TokenStream {
    tokens: Peekable<IntoIter<Token>>,
}

impl TokenStream {
    pub fn new<'src>(src: &'src str) -> Self {
        let tokens: Vec<Token> = TokenKind::lexer(src)
            .spanned()
            .filter_map(|(t, s)| {
                t.ok().map(|t| Token {
                    kind: t,
                    span: s.into(),
                })
            })
            .collect();
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    /// Fetches the next token from the [`Lexer`].
    fn fetch_token(&mut self) -> Token {
        match self.logos.next().map(|t| (t, self.logos.span())) {
            None => Token {
                kind: T![EOF],
                span: Span::new(0, 0),
            },
            Some((t, s)) => match t {
                Ok(T![;]) => self.fetch_token(),
                Ok(kind) => Token {
                    kind,
                    span: Span::new(s.start as u32, s.end as u32),
                },
                Err(e) => {
                    self.errors
                        .push(Error::from(format!("Lexer error: {:?}", e)));
                    Token {
                        kind: T![EOF],
                        span: Span::new(s.start as u32, s.end as u32),
                    }
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

    pub fn at(&mut self, kind: TokenKind) -> bool {
        self.peek().map(|t| t.kind == kind).unwrap_or(false)
    }

    pub fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.next();
            true
        } else {
            false
        }
    }
}
