use std::{fmt::Display, iter::Sum, ops::Add};

use num_bigint::BigInt;
use num_rational::Rational64;

use crate::interpreter::runtime_error::RuntimeError;

use self::env::Env;

pub mod env;

// A Lisp S-expression
#[repr(C)]
#[derive(Debug, Clone)]
pub enum Sexpr {
    // A Lisp atom
    Atom(Atom),

    // A Lisp list represented as a singly-linked list of conses
    List(Option<Box<Cons>>),

    // A Lisp lambda only constructed in eval
    Lambda { args: Vec<String>, body: Box<Sexpr> },

    // A native Rust function only constructed in env
    NativeFn(fn(env: Box<Env>, args: Vec<Sexpr>) -> Result<Sexpr, RuntimeError>),
}

pub const NIL: Sexpr = Sexpr::List(None);

#[derive(Debug, Clone)]
pub struct Cons(pub Box<Sexpr>, pub Option<Box<Cons>>);

impl Display for Cons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.1 {
            Some(cdr) => write!(f, "{} {}", self.0.as_ref(), cdr.as_ref()),
            None => write!(f, "{}", self.0.as_ref()),
        }
    }
}

impl std::fmt::Display for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Sexpr::Atom(a) => write!(f, "{}", a),
            Sexpr::List(head) => {
                if let Some(h) = head {
                    write!(f, "({})", h.as_ref())
                } else {
                    write!(f, "Nil")
                }
            }
            Sexpr::Lambda { args, body } => todo!(),
            Sexpr::NativeFn(_) => write!(f, "NativeFn"),
        }
    }
}

impl PartialEq for Sexpr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Sexpr::Atom(a1), Sexpr::Atom(a2)) => *a1 == *a2,
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
            Lit::Str(s) => write!(f, "\"{}\"", s),
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

impl Add for Number {
    type Output = Result<Number, RuntimeError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Fixnum(l), Number::Fixnum(r)) => {
                if let Some(sum) = l.checked_add(r) {
                    Ok(Number::Fixnum(sum))
                } else {
                    Ok(Number::Bignum(BigInt::from(l) + r))
                }
            }
            (Number::Fixnum(l), Number::Float(r)) => Ok(Number::Float(l as f64 + r)),
            (Number::Fixnum(l), Number::Rational(r)) => {
                Ok(Number::Rational(Rational64::from(l) + r))
            }
            (Number::Fixnum(l), Number::Bignum(r)) => Ok(Number::Bignum(l + r)),
            (Number::Float(l), Number::Fixnum(r)) => Ok(Number::Float(l + r as f64)),
            (Number::Float(l), Number::Float(r)) => Ok(Number::Float(l + r)),
            (Number::Float(l), Number::Rational(r)) => {
                Ok(Number::Float(l + *r.numer() as f64 / *r.denom() as f64))
            }
            (Number::Rational(l), Number::Fixnum(r)) => {
                Ok(Number::Rational(l + Rational64::from(r)))
            }
            (Number::Rational(l), Number::Float(r)) => {
                Ok(Number::Float(*l.numer() as f64 / *l.denom() as f64 + r))
            }
            (Number::Rational(l), Number::Rational(r)) => Ok(Number::Rational(l + r)),
            (Number::Bignum(l), Number::Fixnum(r)) => Ok(Number::Bignum(l + r)),
            (Number::Bignum(l), Number::Bignum(r)) => Ok(Number::Bignum(l + r)),
            (Number::Bignum(_), Number::Float(_))
            | (Number::Float(_), Number::Bignum(_))
            | (Number::Bignum(_), Number::Rational(_))
            | (Number::Rational(_), Number::Bignum(_)) => Err(RuntimeError::NumberOverflowError),
        }
    }
}
