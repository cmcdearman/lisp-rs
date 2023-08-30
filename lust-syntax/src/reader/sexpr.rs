use lust_util::{intern::InternedString, list::List, span::Spanned};
use num_rational::Rational64;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    pub sexprs: Vec<Spanned<Sexpr>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sexpr {
    Atom(Spanned<Atom>),
    Cons(List<Spanned<Sexpr>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Lit(Lit),
    Symbol(InternedString),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(i64),
    Rational(Rational64),
    Real(f64),
    Char(char),
    String(InternedString),
}
