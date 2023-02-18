use std::fmt::Display;

use num_bigint::BigInt;
use num_rational::Rational64;

pub mod env;

#[derive(Debug, Clone)]
pub enum Sexpr {
    Atom(Atom),
    Cons(Box<Self>, Box<Self>),
    Nil,
}

impl std::fmt::Display for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Sexpr::Atom(a) => write!(f, "{}", a),
            Sexpr::Cons(car, cdr) => write!(f, "{} {}", car, cdr),
            Sexpr::Nil => f.write_str("Nil"),
        }
    }
}

impl PartialEq for Sexpr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Sexpr::Atom(a1), Sexpr::Atom(a2)) => *a1 == *a2,
            (Sexpr::Atom(_), Sexpr::Cons(_, _)) | (Sexpr::Cons(_, _), Sexpr::Atom(_)) => todo!(),
            _ => false,
        }
    }
}

impl Eq for Sexpr {}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Sym(String),
    Lit(Lit),
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Sym(s) => write!(f, "{}", s),
            Atom::Lit(l) => write!(f, "{}", l),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Number(Number),
    Bool(bool),
    Str(String),
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lit::Number(n) => write!(f, "{}", n),
            Lit::Bool(b) => write!(f, "{}", b),
            Lit::Str(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Fixnum(i64),
    Float(f64),
    Rational(Rational64),
    Bignum(BigInt),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Fixnum(n) => write!(f, "{}", n),
            Number::Float(d) => write!(f, "{}", d),
            Number::Rational(r) => write!(f, "{}", r),
            Number::Bignum(b) => write!(f, "{}", b),
        }
    }
}
