use std::fmt;
use std::iter::Peekable;
use std::ops::{Index, Range};
use std::vec::IntoIter;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum LogosToken {
    #[regex(r##"([A-Za-z]|_)([A-Za-z]|_|\d)*"##)]
    Ident,
    #[regex(r#"((\d+(\.\d+)?)|(\.\d+))([Ee](\+|-)?\d+)?"#)]
    Number,
    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    String,

    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Quo,
    #[token("%")]
    Mod,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    #[token("let")]
    Let,
    #[token("lambda")]
    Lambda,

    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,
    #[error]
    Error,
    #[regex(r#"//[^\n]*"#)]
    Comment,
    Eof,
}

impl LogosToken {
    #[rustfmt::skip]
    pub fn kind(&self) -> TokenKind {
        use LogosToken::*;
        use crate::T;
        match self {
            Ident => T![ident],
            String => T![string],
            Number => T![number],
            Add => T![+],
            Sub => T![-],
            Mul => T![*],
            Quo => T![/],
            Mod => T![%],
            LParen => T!['('],
            RParen => T![')'],
            Let => T![let],
            Lambda => T![lambda],
            Comment => T![comment],
            Whitespace => T![ws],
            Error => T![error],
            Eof => T![EOF]
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenKind {
    Error,
    Eof,
    Comment,
    Whitespace,

    // Literals
    Ident,
    Number,
    String,

    // Operators
    Add,    // +
    Sub,    // -
    Mul,    // *
    Quo,    // /
    Mod,    // %

    // Parentheses
    LParen, // (
    RParen, // )

    // Keywords
    Let,    // let
    Lambda, // lambda
}

#[macro_export]
macro_rules! T {
    [ws] => {
        $crate::token::TokenKind::Whitespace
    };
    [error] => {
        $crate::token::TokenKind::Error
    };
    [EOF] => {
        $crate::token::TokenKind::Eof
    };
    [comment] => {
        $crate::token::TokenKind::Comment
    };
    [ident] => {
        $crate::token::TokenKind::Ident
    };
    [number] => {
        $crate::token::TokenKind::Number
    };
    [string] => {
        $crate::token::TokenKind::String
    };
    [+] => {
        $crate::token::TokenKind::Add
    };
    [-] => {
        $crate::token::TokenKind::Sub
    };
    [*] => {
        $crate::token::TokenKind::Mul
    };
    [/] => {
        $crate::token::TokenKind::Quo
    };
    [%] => {
        $crate::token::TokenKind::Mod
    };
    ['('] => {
        $crate::token::TokenKind::LParen
    };
    [')'] => {
        $crate::token::TokenKind::RParen
    };
    [let] => {
        $crate::token::TokenKind::Let
    };
    [lambda] => {
        $crate::token::TokenKind::Lambda
    };
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                T![ws] => "Whitespace",
                T![error] => "Error",
                T![EOF] => "<EOF>",
                T![comment] => "Comment",
                T![ident] => "Ident",
                T![number] => "Number",
                T![string] => "String",
                T![+] => "+",
                T![-] => "-",
                T![*] => "*",
                T![/] => "/",
                T![%] => "%",
                T!['('] => "(",
                T![')'] => ")",
                T![let] => "let",
                T![lambda] => "lambda",
            }
        )
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Default, Debug)]
pub struct Span {
    pub start: u32,
    pub end: u32
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
            end:   range.end as u32,
        }
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[Range::<usize>::from(index)]
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}

impl Token {
    pub fn len(&self) -> usize {
        (self.span.end - self.span.start) as usize
    }

    pub fn text<'input>(&self, input: &'input str) -> &'input str {
        &input[self.span]
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} - <{}, {}>", self.kind, self.span.start, self.span.end)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

pub struct TokenStream {
    token_iter: Peekable<IntoIter<Token>>
}

impl TokenStream  {
    pub fn new(tokens: Vec<Token>) -> Self { Self { token_iter: tokens.into_iter().peekable() } }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_token = self.token_iter.next()?;
            if !matches!(next_token.kind, T![comment]) {
                return Some(next_token);
            }
        }
    }
}