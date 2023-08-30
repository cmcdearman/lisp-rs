use super::token::Token;
use lust_util::span::{Span, Spanned};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxError {
    LexerError,
    UnmatchedParen(Span),
    UnexpectedToken(Token),
    LitParseError(Token),
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxError::LexerError => write!(f, "Lexer error"),
            SyntaxError::UnmatchedParen(s) => write!(f, "Unmatched paren at {}", s),
            SyntaxError::UnexpectedToken(t) => {
                write!(f, "Unexpected token {:?} at {}", t.value, t.span)
            }
            SyntaxError::LitParseError(t) => write!(f, "Failed to parse literal at {}", t.span),
        }
    }
}

pub type ReaderError = Spanned<SyntaxError>;
pub type ReadResult<T> = std::result::Result<T, ReaderError>;
