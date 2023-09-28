use lust_util::{intern::InternedString, span::Spanned};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxError(pub InternedString);

impl SyntaxError {
    pub fn new(msg: &str) -> Self {
        Self(msg.into())
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        InternedString::from(self.0.key).fmt(f)
    }
}

impl From<String> for SyntaxError {
    fn from(s: String) -> Self {
        Self(InternedString::from(&*s))
    }
}

pub type ParserError = Spanned<SyntaxError>;
pub type ParseResult<T> = std::result::Result<T, ParserError>;
