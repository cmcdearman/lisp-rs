use num_rational::Rational64;

use crate::{intern::InternedString, list::List};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Symbol(InternedString),
    Lit(Lit),
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

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(i64),
    Real(f64),
    Rational(Rational64),
    Bool(bool),
    Char(char),
    String(InternedString),
}
