use std::fmt::Display;

use crate::{interner::InternedString, list::List};

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
    Symbol(InternedString),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParserError(pub InternedString);

impl ParserError {
    pub fn new(msg: &str) -> Self {
        Self(msg.into())
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", InternedString::from(self.0.key))
    }
}

pub type Result<T> = std::result::Result<T, ParserError>;
