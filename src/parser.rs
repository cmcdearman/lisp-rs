use crate::list::List;

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Atom(Atom),
    List(List<Self>),
    Lambda {
        param: Vec<Self>,
        body: Box<Self>,
    },
    Apply {
        func: Box<Self>,
        arg: Box<Self>,
    },
    Let {
        name: String,
        value: Box<Self>,
        body: Box<Self>,
    },
    If {
        cond: Box<Self>,
        then: Box<Self>,
        else_: Box<Self>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Number(Number),
    Symbol(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParserErrorKind {
    ParseIntegerError,
    ParseFloatError,
    ParseRationalError,
    ParseStringError,
    UnexpectedEofError,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParserError {
    kind: ParserErrorKind,
    span: Span,
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = std::result::Result<T, ParserError>;
