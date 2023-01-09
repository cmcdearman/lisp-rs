use logos::Logos;
use std::fmt;
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
        &self[Range::from(index)]
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

#[derive(Debug, Clone)]
pub struct TokenStream {
    token_iter: IntoIter<Token>,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            token_iter: tokens.into_iter(),
        }
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_iter.next()
    }
}
