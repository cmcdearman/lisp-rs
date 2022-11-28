use logos::Logos;
use std::fmt;
use std::iter::Peekable;
use std::ops::{Index, Range};
use std::vec::IntoIter;

#[derive(Logos, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenKind {
    #[regex(r##"([A-Za-z]|_)([A-Za-z]|_|\d)*"##)]
    Ident,
    #[regex(r#"((\d+(\.\d+)?)|(\.\d+))([Ee](\+|-)?\d+)?"#)]
    Num,
    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    String,
    #[regex(r#"(true|false)"#)]
    Bool,

    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Quo,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    #[token("let")]
    Let,
    #[token("fn")]
    Fn,
    #[token("def")]
    Def,
    #[token("apply")]
    Apply,
    #[token("mod")]
    Mod,
    #[token("cond")]
    Cond,

    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,
    #[error]
    Err,
    #[regex(r#";[^\n]*"#)]
    Comment,
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenKind::Whitespace => "Whitespace",
                TokenKind::Err => "Error",
                TokenKind::Eof => "<EOF>",
                TokenKind::Comment => "Comment",
                TokenKind::Ident => "Ident",
                TokenKind::Num => "Number",
                TokenKind::String => "String",
                TokenKind::Bool => "Bool",
                TokenKind::Add => "+",
                TokenKind::Sub => "-",
                TokenKind::Mul => "*",
                TokenKind::Quo => "/",
                TokenKind::LParen => "(",
                TokenKind::RParen => ")",
                TokenKind::Let => "let",
                TokenKind::Fn => "fn",
                TokenKind::Def => "def",
                TokenKind::Apply => "apply",
                TokenKind::Mod => "mod",
                TokenKind::Cond => "cond",
            }
        )
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Default, Debug)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
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
        &self[Range::<usize>::from(index)]
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lit: String,
}

impl Token {
    pub fn len(&self) -> usize {
        (self.span.end - self.span.start) as usize
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} - <{}, {}>",
            self.kind, self.span.start, self.span.end
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

pub struct TokenStream {
    token_iter: Peekable<IntoIter<Token>>,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            token_iter: tokens.into_iter().peekable(),
        }
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_token = self.token_iter.next()?;
            if !matches!(next_token.kind, TokenKind::Comment) {
                return Some(next_token);
            }
        }
    }
}
