use crate::span::{Span, Spanned};
use std::fmt::Display;

use super::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum ReaderError {
    LexerError,
    UnmatchedParen(Span),
    UnexpectedToken(Spanned<Token>),
}

impl Display for ReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReaderError::LexerError => write!(f, "Lexer error"),
            ReaderError::UnmatchedParen(s) => write!(f, "Unmatched paren at {}", s),
            ReaderError::UnexpectedToken((t, s)) => write!(f, "Unexpected token {:?} at {}", t, s),
        }
    }
}

pub type ReadResult<T> = std::result::Result<T, ReaderError>;
