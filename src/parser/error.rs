#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParserError(pub String);

impl ParserError {
    pub fn new(msg: String) -> Self {
        Self(msg)
    }
}

pub type Result<T> = std::result::Result<T, ParserError>;