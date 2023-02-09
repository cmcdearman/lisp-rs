use logos::Logos;
use std::fmt::{self, Display};
use std::ops::{Index, Range};
use std::vec::IntoIter;

#[derive(Logos, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenKind {
    #[regex(r#"[^\[\]()\s]+"#)]
    Ident,
    #[regex(r#"\d+"#, priority = 3)]
    Int,
    #[regex(r#"((\d+(\.\d+)?)|(\.\d+))([Ee](\+|-)?\d+)?"#, priority = 2)]
    Float,
    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    String,
    #[regex(r#"(true|false)"#)]
    Bool,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBrack,
    #[token("]")]
    RBrack,
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
                TokenKind::Int => "Int",
                TokenKind::Float => "Float",
                TokenKind::String => "String",
                TokenKind::Bool => "Bool",
                TokenKind::LParen => "(",
                TokenKind::RParen => ")",
                TokenKind::LBrack => "[",
                TokenKind::RBrack => "]",
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} - {}", self.kind, self.span)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

