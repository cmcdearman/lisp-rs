use super::token::Token;
use lust_util::span::Span;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum ReaderError {
    LexerError,
    UnmatchedParen(Span),
    UnexpectedToken(Token),
}

impl Display for ReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReaderError::LexerError => write!(f, "Lexer error"),
            ReaderError::UnmatchedParen(s) => write!(f, "Unmatched paren at {}", s),
            ReaderError::UnexpectedToken(t) => {
                write!(f, "Unexpected token {:?} at {}", t.value, t.span)
            }
        }
    }
}

pub type ReadResult<T> = std::result::Result<T, ReaderError>;
