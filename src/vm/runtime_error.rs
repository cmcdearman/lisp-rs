#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeError(pub String);

impl RuntimeError {
    pub fn new(msg: String) -> RuntimeError {
        RuntimeError(msg)
    }
}

impl From<&str> for RuntimeError {
    fn from(msg: &str) -> RuntimeError {
        RuntimeError::new(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;
