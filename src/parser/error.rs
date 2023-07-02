use std::fmt::Display;

use crate::intern::InternedString;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Error(pub InternedString);

impl Error {
    pub fn new(msg: &str) -> Self {
        Self(msg.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        InternedString::from(self.0.key).fmt(f)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self(InternedString::from(&*s))
    }
}

pub type ParseResult<T> = std::result::Result<T, Error>;
