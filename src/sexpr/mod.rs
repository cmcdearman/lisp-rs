use std::{fmt::{Display, Debug}, rc::Rc, cell::RefCell, collections::HashMap};

use self::{symbol::Symbol, list::List, lambda::Lambda, env::Env, number::Number};

pub mod cons;
pub mod env;
pub mod lambda;
pub mod list;
pub mod number;
pub mod symbol;

#[derive(Debug, Clone)]
pub enum Sexpr {
    Atom(Atom),
    List(List),
    Lambda(Lambda),
    NativeFn(NativeFn),
}

impl std::fmt::Display for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Sexpr::Atom(a) => write!(f, "{}", a),
            Sexpr::List(l) => write!(f, "{}", l),
            Sexpr::Lambda(l) => write!(f, "{}", l),
            Sexpr::NativeFn(_) => f.write_str("<native_function>"),
        }
    }
}

impl PartialEq for Sexpr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Sexpr::Atom(_), Sexpr::Atom(_)) => todo!(),
            (Sexpr::Atom(_), Sexpr::List(_)) => todo!(),
            (Sexpr::Atom(_), Sexpr::Lambda(_)) => todo!(),
            (Sexpr::Atom(_), Sexpr::NativeFn(_)) => todo!(),
            (Sexpr::List(_), Sexpr::Atom(_)) => todo!(),
            (Sexpr::List(l1), Sexpr::List(l2)) => l1 == l2,
            (Sexpr::Lambda(l), Sexpr::Lambda(m)) => l == m,
            (Sexpr::NativeFn(f), Sexpr::NativeFn(g)) => f == g,
            _ => false
        }
    }
}

impl Eq for Sexpr {}

#[derive(Debug, Clone)]
pub enum Atom {
    Sym(Symbol),
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

#[derive(Debug, Clone)]
pub enum Lit {
    Number(Number),
    Bool(bool),
    Str(String),
    // Vec(Vec<Object>),
    // HashMap(HashMap<Object, Object>),
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lit::Number(n) => write!(f, "{}", n),
            Lit::Bool(b) => write!(f, "{}", b),
            Lit::Str(s) => write!(f, "{}", s),
            // Lit::Vec(v) => write!(f, "{}", v),
            // Lit::HashMap(_) => todo!(),
        }
    }
}

pub type NativeFn = fn(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr, String>;