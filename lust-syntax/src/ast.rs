use num_rational::Rational64;

use crate::{intern::InternedString, span::Spanned};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Symbol(InternedString),
    Lit(Lit),
    List(Vec<Spanned<Self>>),
    BinaryOp {
        op: BinaryOp,
        lhs: Box<Spanned<Self>>,
        rhs: Box<Spanned<Self>>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Spanned<Self>>,
    },
    Lambda {
        params: Vec<Spanned<Self>>,
        body: Box<Spanned<Self>>,
    },
    Apply {
        func: Box<Spanned<Self>>,
        args: Vec<Spanned<Self>>,
    },
    Let {
        name: Spanned<InternedString>,
        value: Box<Spanned<Self>>,
        body: Box<Spanned<Self>>,
    },
    If {
        cond: Box<Spanned<Self>>,
        then: Box<Spanned<Self>>,
        else_: Box<Spanned<Self>>,
    },
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(i64),
    Rational(Rational64),
    Real(f64),
    Char(char),
    String(InternedString),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
}
