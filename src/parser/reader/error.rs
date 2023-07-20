use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReaderError {
    UnmatchedParen,
    LexerError,
}

impl Display for ReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReaderError::UnmatchedParen => write!(f, "Unmatched paren"),
            ReaderError::LexerError => write!(f, "Lexer error"),
        }
    }
}

pub type ReadResult<T> = std::result::Result<T, ReaderError>;
