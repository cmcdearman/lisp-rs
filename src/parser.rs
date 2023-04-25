use std::fmt::{Debug, Display};

use num_bigint::BigInt;
use num_rational::{BigRational, Rational64};

use crate::{
    interner::InternedString,
    list::List,
    reader::{self, Cons, Sexpr},
};

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
    Unit,
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

// Parser entry point
pub fn parse_expr(sexpr: &Sexpr) -> Result<Expr> {
    match sexpr {
        Sexpr::Atom(a) => match a {
            reader::Atom::Lit(l) => match l {
                reader::Lit::Number(n) => Ok(Expr::Atom(Atom::Lit(Lit::Number(match n {
                    reader::Number::Int(i) => Number::Int(*i),
                    reader::Number::BigInt(b) => Number::BigInt(b.clone()),
                    reader::Number::Float(f) => Number::Float(*f),
                    reader::Number::Rational(r) => Number::Rational(r.clone()),
                    reader::Number::BigRational(br) => Number::BigRational(br.clone()),
                })))),
                reader::Lit::String(s) => Ok(Expr::Atom(Atom::Lit(Lit::String(s.clone())))),
                reader::Lit::Char(c) => Ok(Expr::Atom(Atom::Lit(Lit::Char(*c)))),
                reader::Lit::Bool(b) => Ok(Expr::Atom(Atom::Lit(Lit::Bool(*b)))),
            },
            reader::Atom::Symbol(s) => Ok(Expr::Atom(Atom::Symbol(s.clone()))),
        },
        Sexpr::List(l) => match l.clone() {
            reader::List { head: None } => Ok(Expr::Unit),
            reader::List { head: Some(h) } => match *h.clone() {
                Cons {
                    car: sexpr,
                    cdr: None,
                } => parse_expr(&sexpr),
                Cons {
                    car: sexpr,
                    cdr: Some(cdr),
                } => match sexpr {
                    Sexpr::Atom(a) => match a {
                        reader::Atom::Symbol(s) => match &*s.to_string() {
                            "lambda" => parse_lambda(&cdr),
                            "let" => parse_let(&cdr),
                            "if" => parse_if(&cdr),
                            _ => parse_apply(&sexpr, &cdr),
                        },
                        _ => parse_apply(&sexpr, &cdr),
                    },
                    _ => parse_apply(&sexpr, &cdr),
                },
            },
        },
    }
}
