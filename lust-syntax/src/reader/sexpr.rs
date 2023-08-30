use lust_util::{intern::InternedString, list::List, span::Spanned};
use num_rational::Rational64;
use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub struct Root {
    pub sexprs: Vec<Spanned<Sexpr>>,
}

#[derive(Clone, PartialEq)]
pub enum Sexpr {
    Atom(Spanned<Atom>),
    Cons(List<Spanned<Sexpr>>),
}

impl Debug for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(a) => write!(f, "{:?}", a),
            Self::Cons(l) => write!(f, "{:?}", *l),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Atom {
    Lit(Spanned<Lit>),
    Symbol(Spanned<InternedString>),
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
    Int(i64),
    Rational(Rational64),
    Real(f64),
    Char(char),
    String(InternedString),
}

impl Debug for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{:?}", i),
            Self::Rational(r) => write!(f, "{:?}", r),
            Self::Real(r) => write!(f, "{:?}", r),
            Self::Char(c) => write!(f, "{:?}", c),
            Self::String(s) => write!(f, "{:?}", s),
        }
    }
}
