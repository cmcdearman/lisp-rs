use crate::interner::InternedString;
use num_bigint::BigInt;
use num_rational::{BigRational, Rational64};
use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub enum Sexpr {
    Atom(Atom),
    List(ConsList),
}

impl Debug for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(a) => write!(f, "{:?}", a),
            Self::List(l) => write!(f, "{:?}", l),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ConsList {
    pub head: Option<Box<Cons>>,
}

impl ConsList {
    pub const NIL: Self = Self { head: None };

    pub fn new(head: Option<Box<Cons>>) -> Self {
        Self { head }
    }
}

impl Debug for ConsList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.head {
            Some(h) => write!(f, "{:?}", h),
            None => write!(f, "Nil"),
        }
    }
}

impl IntoIterator for ConsList {
    type Item = Sexpr;

    type IntoIter = ConsIter;

    fn into_iter(self) -> Self::IntoIter {
        ConsIter(self.head.clone())
    }
}

#[derive(Clone, PartialEq)]
pub struct Cons {
    pub car: Sexpr,
    pub cdr: Option<Box<Cons>>,
}

impl Debug for Cons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.cdr {
            Some(cdr) => write!(f, "({:?} . {:?})", self.car, cdr.as_ref()),
            None => write!(f, "({:?} . Nil)", self.car),
        }
    }
}

#[derive(Clone)]
pub struct ConsIter(Option<Box<Cons>>);

impl Iterator for ConsIter {
    type Item = Sexpr;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.clone().map(|cons| {
            let sexpr = cons.car.clone();

            self.0 = cons.cdr.clone();

            sexpr
        })
    }
}

// impl ExactSizeIterator for ConsIter {
//     fn len(&self) -> usize {
//         self.count()
//     }
// }

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
    Int(i64),
    BigInt(BigInt),
    Real(f64),
    Rational(Rational64),
    BigRational(BigRational),
    Bool(bool),
    Char(char),
    String(InternedString),
}

impl Debug for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{:?}", i),
            Self::BigInt(i) => write!(f, "{:?}", i),
            Self::Real(r) => write!(f, "{:?}", r),
            Self::Rational(r) => write!(f, "{:?}", r),
            Self::BigRational(r) => write!(f, "{:?}", r),
            Self::String(s) => write!(f, "{:?}", s),
            Self::Char(c) => write!(f, "{:?}", c),
            Self::Bool(b) => write!(f, "{:?}", b),
        }
    }
}
