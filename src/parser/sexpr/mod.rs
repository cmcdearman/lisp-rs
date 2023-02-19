use std::{fmt::Display, ops::Add};

use num_bigfloat::BigFloat;
use num_bigint::BigInt;
use num_rational::{BigRational, Rational64};

use self::env::Env;

pub mod env;

/*
 * A Lisp S-expression
 */
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
    NativeFn(fn(env: Box<Env>, args: Vec<Sexpr>) -> Result<Sexpr, String>),
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
    Bigfloat(BigFloat),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Fixnum(n) => write!(f, "{}", n),
            Number::Float(d) => write!(f, "{}", d),
            Number::Rational(r) => write!(f, "{}", r),
            Number::Bignum(b) => write!(f, "{}", b),
            Number::Bigfloat(b) => write!(f, "{}", b),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Fixnum(l), Number::Fixnum(r)) => Number::Fixnum(l + r),
            (Number::Fixnum(l), Number::Float(r)) => Number::Float(l as f64 + r),
            (Number::Fixnum(l), Number::Rational(r)) => Number::Rational(Rational64::from(l) + r),
            (Number::Fixnum(l), Number::Bignum(r)) => Number::Bignum(l + r),
            (Number::Float(l), Number::Fixnum(r)) => Number::Float(l + r as f64),
            (Number::Float(l), Number::Float(r)) => Number::Float(l + r),
            (Number::Float(l), Number::Rational(r)) => {
                Number::Float(l + *r.numer() as f64 / *r.denom() as f64)
            }
            (Number::Float(l), Number::Bignum(r)) => {
                let bigr = BigRational::from(r);
                Number::Bigfloat(BigFloat::from_f64(l) + BigFloat::from(r))
            }
            (Number::Rational(l), Number::Fixnum(r)) => Number::Rational(l + Rational64::from(r)),
            (Number::Rational(l), Number::Float(r)) => {
                Number::Float(*l.numer() as f64 / *l.denom() as f64 + r)
            }
            (Number::Rational(l), Number::Rational(r)) => todo!(),
            (Number::Rational(l), Number::Bignum(r)) => todo!(),
            (Number::Bignum(l), Number::Fixnum(r)) => todo!(),
            (Number::Bignum(l), Number::Float(r)) => todo!(),
            (Number::Bignum(l), Number::Rational(r)) => todo!(),
            (Number::Bignum(l), Number::Bignum(r)) => todo!(),
            (Number::Fixnum(_), Number::Bigfloat(_)) => todo!(),
            (Number::Float(_), Number::Bigfloat(_)) => todo!(),
            (Number::Rational(_), Number::Bigfloat(_)) => todo!(),
            (Number::Bignum(_), Number::Bigfloat(_)) => todo!(),
            (Number::Bigfloat(_), Number::Fixnum(_)) => todo!(),
            (Number::Bigfloat(_), Number::Float(_)) => todo!(),
            (Number::Bigfloat(_), Number::Rational(_)) => todo!(),
            (Number::Bigfloat(_), Number::Bignum(_)) => todo!(),
            (Number::Bigfloat(_), Number::Bigfloat(_)) => todo!(),
        }
    }
}


