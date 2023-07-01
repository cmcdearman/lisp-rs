use crate::intern::InternedString;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(pub InternedString);

impl Error {
    pub fn new(msg: &str) -> Self {
        Self(InternedString::from(msg))
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

pub type ReadResult<T> = std::result::Result<T, Error>;
