use std::fmt::{Debug, Display};

use num_bigint::BigInt;
use num_rational::{BigRational, Rational64};

use crate::{interner::InternedString, list::List, reader::Sexpr};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
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
        name: InternedString,
        value: Box<Self>,
        body: Box<Self>,
    },
    If {
        cond: Box<Self>,
        then: Box<Self>,
        else_: Box<Self>,
    },
}

#[derive(Clone, PartialEq)]
pub enum Atom {
    Lit(Lit),
    Symbol(InternedString),
}

impl Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lit(l) => write!(f, "{:?}", l),
            Self::Symbol(s) => write!(f, "{:?}", s),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Lit {
    Number(Number),
    String(InternedString),
    Char(char),
    Bool(bool),
}

impl Debug for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{:?}", n),
            Self::String(s) => write!(f, "{:?}", s),
            Self::Char(c) => write!(f, "{:?}", c),
            Self::Bool(b) => write!(f, "{:?}", b),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(i64),
    BigInt(BigInt),
    Float(f64),
    // BigFloat(),
    Rational(Rational64),
    BigRational(BigRational),
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

pub fn parse(sexpr: &Sexpr) -> Result<Expr> {
    match sexpr {
        Sexpr::Atom(_) => todo!(),
        Sexpr::List(_) => todo!(),
    }
}
