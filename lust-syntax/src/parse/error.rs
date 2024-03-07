use std::fmt::Display;

use lust_utils::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    msg: String,
    span: Span,
}

impl ParseError {
    pub fn new(msg: String, span: Span) -> Self {
        Self { msg, span }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.span, self.msg)
    }
}

pub type ParseResult<T> = std::result::Result<T, ParseError>;
