use lust_util::intern::InternedString;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParserError(pub InternedString);

impl ParserError {
    pub fn new(msg: &str) -> Self {
        Self(msg.into())
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        InternedString::from(self.0.key).fmt(f)
    }
}

impl From<String> for ParserError {
    fn from(s: String) -> Self {
        Self(InternedString::from(&*s))
    }
}

pub type ParseResult<T> = std::result::Result<T, ParserError>;
