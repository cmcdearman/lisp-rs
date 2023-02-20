use std::fmt::Display;

use super::lexer::token::Span;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParserError {
    kind: ParserErrorKind,
    span: Span,
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParserErrorKind {
    ParseIntegerError,
    ParseFloatError,
    ParseRationalError,
    ParseStringError,
    UnexpectedEofError,
}
