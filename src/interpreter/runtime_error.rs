use std::fmt::Display;

use crate::parser::parser_error::ParserError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuntimeError {
    NumberOverflowError,
    IvalidFunctionArgumentsError,
    UnknownIdentError,
    EarlyListEndError,
    FirstElemError,
    ParserError(ParserError),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;
