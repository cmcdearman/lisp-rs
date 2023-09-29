#[derive(Clone, PartialEq)]
pub struct Format<T> {
    pub indent: usize,
    pub value: T,
}

impl<T> Format<T> {
    pub fn new(indent: usize, value: T) -> Self {
        Self { indent, value }
    }
}

pub fn spaces(n: usize) -> String {
    " ".repeat(n)
}
