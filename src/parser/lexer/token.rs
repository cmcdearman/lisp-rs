use logos::Logos;
use std::fmt::{self, Display};
use std::ops::{Index, Range};

#[derive(Logos, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum TokenKind {
    #[default]
    Eof,
    #[error]
    Err,
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,
    #[regex(r#";[^\n]*"#)]
    Comment,
    #[regex(r#"[^\[\]()\s]+"#)]
    Ident,
    #[regex(r#"(\+|-)?\d+(i8|u8|i16|u16|i32|u32|i64|u64)?"#, priority = 3)]
    Int,
    #[regex(
        r#"(\+|-)?((\d+(\.\d+)?)|(\.\d+))([Ee](\+|-)?\d+)?(f32|f64)?"#,
        priority = 2
    )]
    Float,
    #[regex(r#"(\+|-)?\d+/\d+"#)]
    Rational,
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
    #[token(":")]
    Colon,
}

#[macro_export]
macro_rules! T {
    [EOF] => {
        $crate::parser::lexer::token::TokenKind::Eof
    };
    [err] => {
        $crate::parser::lexer::token::TokenKind::Err
    };
    [ws] => {
        $crate::parser::lexer::token::TokenKind::Whitespace
    };
    [;] => {
        $crate::parser::lexer::token::TokenKind::Comment
    };
    [ident] => {
        $crate::parser::lexer::token::TokenKind::Ident
    };
    [int] => {
        $crate::parser::lexer::token::TokenKind::Int
    };
    [float] => {
        $crate::parser::lexer::token::TokenKind::Float
    };
    [ratio] => {
        $crate::parser::lexer::token::TokenKind::Rational
    };
    [str] => {
        $crate::parser::lexer::token::TokenKind::String
    };
    [bool] => {
        $crate::parser::lexer::token::TokenKind::Bool
    };
    ['('] => {
        $crate::parser::lexer::token::TokenKind::LParen
    };
    [')'] => {
        $crate::parser::lexer::token::TokenKind::RParen
    };
    ['['] => {
        $crate::parser::lexer::token::TokenKind::LBrack
    };
    [']'] => {
        $crate::parser::lexer::token::TokenKind::RBrack
    };
    [:] => {
        $crate::parser::lexer::token::TokenKind::Colon
    };
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                T![EOF] => "<EOF>",
                T![err] => "Error",
                T![ws] => "Whitespace",
                T![;] => "Comment",
                T![ident] => "Ident",
                T![int] => "Int",
                T![float] => "Float",
                T![ratio] => "Rational",
                T![str] => "String",
                T![bool] => "Bool",
                T!['('] => "(",
                T![')'] => ")",
                T!['['] => "[",
                T![']'] => "]",
                T![:] => ":",
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
