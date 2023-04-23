use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    ops::{Add, Div, Mul, Rem, Sub},
    ptr::hash,
    rc::Rc,
};

use itertools::{join, Itertools};
use num_bigint::BigInt;
use num_rational::{BigRational, Rational64};
use serde::Serialize;

use crate::interpreter::runtime_error::{Result, RuntimeError};

use self::env::Env;

pub mod env;

// A Lisp S-expression
#[repr(C)]
#[derive(Clone)]
pub enum Sexpr {
    // A Lisp atom
    Atom(Atom),

    // A Lisp list represented as a singly-linked list of conses
    List(List),

    // A Lisp lambda only constructed in eval
    Lambda {
        env: Rc<RefCell<Env>>,
        args: Vec<Self>,
        body: Box<Self>,
    },

    // A native Rust function only constructed in env
    NativeFn(fn(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr>),
    // A Lisp environment
    // Env(Env),
}

impl Sexpr {
    pub fn is_special_form(&self) -> bool {
        if let Sexpr::Atom(Atom::Sym(s)) = &self {
            return match s.as_str() {
                "def" | "let" | "fn" | "quote" | "if" => true,
                _ => false,
            };
        }
        false
    }

    pub fn get_special_form(&self) -> Option<String> {
        if let Sexpr::Atom(Atom::Sym(s)) = &self {
            return match s.as_str() {
                "def" | "let" | "fn" | "quote" | "if" | "cond" => Some(s.clone()),
                _ => None,
            };
        }
        None
    }
}

impl Display for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Atom(a) => write!(f, "{}", a),
            Self::List(head) => write!(f, "{}", head),
            Self::Lambda { env, args, body } => write!(f, "<#fn>"),
            Self::NativeFn(_) => write!(f, "NativeFn"),
        }
    }
}

impl Debug for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(a) => write!(f, "{:?}", a),
            Self::List(l) => write!(f, "{:?}", l),
            Self::Lambda { env, args, body } => write!(f, "<#fn({:?})>", args),
            Self::NativeFn(nf) => write!(f, "{:?}", nf),
            // Self::Env(e) => write!(f, "<#env>"),
        }
    }
}

impl PartialEq for Sexpr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Atom(a1), Self::Atom(a2)) => *a1 == *a2,
            _ => false,
        }
    }
}

impl Eq for Sexpr {}

pub const NIL: Sexpr = Sexpr::List(List { head: None });

#[derive(Clone, Default, PartialEq)]
pub struct List {
    head: Option<Box<Cons>>,
}

impl List {
    pub fn new(head: Option<Box<Cons>>) -> Self {
        Self { head }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.head {
            Some(h) => write!(f, "({})", h),
            None => write!(f, "Nil"),
        }
    }
}

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.head {
            Some(h) => write!(f, "{:?}", h),
            None => write!(f, "Nil"),
        }
    }
}

impl IntoIterator for List {
    type Item = Sexpr;

    type IntoIter = ConsIter;

    fn into_iter(self) -> Self::IntoIter {
        ConsIter(self.head.clone())
    }
}

impl From<Vec<Sexpr>> for List {
    fn from(v: Vec<Sexpr>) -> Self {
        let mut head = None;
        for s in v.into_iter().rev() {
            head = Some(Box::new(Cons { car: s, cdr: head }));
        }
        Self { head }
    }
}

#[derive(Clone, PartialEq)]
pub struct Cons {
    pub car: Sexpr,
    pub cdr: Option<Box<Cons>>,
}

impl Display for Cons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.cdr {
            Some(cdr) => write!(f, "{} {}", self.car, cdr.as_ref()),
            None => write!(f, "{}", self.car),
        }
    }
}

impl Debug for Cons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.cdr {
            Some(cdr) => write!(f, "({} . {:?})", self.car, cdr.as_ref()),
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

impl ExactSizeIterator for ConsIter {
    fn len(&self) -> usize {
        let mut length: usize = 0;

        self.clone().for_each(|_| length += 1);

        length
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Sym(String),
    Keyword(String),
    Lit(Lit),
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Sym(s) => write!(f, "{}", s),
            Atom::Keyword(k) => write!(f, ":{}", k),
            Atom::Lit(l) => write!(f, "{}", l),
        }
    }
}

// An alist map implementation
pub struct Map {
    map: List,
}

impl Map {
    pub fn new() -> Self {
        Self {
            map: List::new(None),
        }
    }

    pub fn insert(&mut self, key: Sexpr, value: Sexpr) {
        self.map.head = Some(Box::new(Cons {
            car: Sexpr::List(List::new(Some(Box::new(Cons {
                car: key,
                cdr: Some(Box::new(Cons {
                    car: value,
                    cdr: None,
                })),
            })))),
            cdr: self.map.head.clone(),
        }));
    }

    pub fn get(&self, key: &Sexpr) -> Option<Sexpr> {
        for cons in self.map.clone() {
            if let Sexpr::List(l) = cons {
                if let Some(Cons {
                    car: k,
                    cdr: Some(Cons { car: v, cdr: None }),
                }) = l.head
                {
                    if k == *key {
                        return Some(v);
                    }
                }
            }
        }
        None
    }

    pub fn remove(&mut self, key: &Sexpr) -> Option<Sexpr> {
        let mut prev = None;
        let mut curr = self.map.head.clone();
        while let Some(cons) = curr {
            if let Sexpr::List(l) = cons.car {
                if let Some(Cons {
                    car: k,
                    cdr: Some(Cons { car: v, cdr: None }),
                }) = l.head
                {
                    if k == *key {
                        if let Some(p) = prev {
                            p.cdr = cons.cdr.clone();
                        } else {
                            self.map.head = cons.cdr.clone();
                        }
                        return Some(v);
                    }
                }
            }
            prev = Some(cons);
            curr = cons.cdr.clone();
        }
        None
    }

    pub fn contains_key(&self, key: &Sexpr) -> bool {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone)]
pub enum Lit {
    Number(Number),
    Bool(bool),
    Str(String),
    Vec(Vec<Sexpr>),
    Map(Rc<RefCell<List>>),
    Set(Rc<RefCell<BTreeSet<Sexpr>>>),
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lit::Number(n) => write!(f, "{}", n),
            Lit::Bool(b) => write!(f, "{}", b),
            Lit::Str(s) => write!(f, "\"{}\"", s),
            Lit::Vec(v) => write!(f, "[{}]", join(v, " ")),
            Lit::Map(m) => write!(
                f,
                "{{{}}}",
                join(m.borrow().iter().map(|(k, v)| format!("{} {}", k, v)), " ")
            ),
            Lit::Set(s) => write!(f, "#{{{}}}", join(s.borrow().iter(), " ")),
        }
    }
}

impl PartialEq for Lit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l), Self::Number(r)) => l == r,
            (Self::Bool(l), Self::Bool(r)) => l == r,
            (Self::Str(l), Self::Str(r)) => l == r,
            (Self::Vec(l), Self::Vec(r)) => l == r,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Number {
    Int(i64),
    BigInt(BigInt),
    Float(f64),
    Rational(Rational64),
    BigRational(BigRational),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Int(n) => write!(f, "{}", n),
            Number::BigInt(b) => write!(f, "{}", b),
            Number::Float(d) => write!(f, "{}", d),
            Number::Rational(r) => write!(f, "{}", r),
            Number::BigRational(br) => write!(f, "{}", br),
        }
    }
}
