use std::{fmt::{Display, Debug}, rc::Rc, cell::RefCell, collections::HashMap};

use self::{symbol::Symbol, list::List, lambda::Lambda, env::Env, number::Number};

pub mod cons;
pub mod env;
pub mod lambda;
pub mod list;
pub mod number;
pub mod symbol;

#[derive(Debug, Clone)]
pub enum Object {
    Atom(Atom),
    List(List),
    Lambda(Lambda),
    NativeFn(NativeFn),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Atom(a) => write!(f, "{}", a),
            Object::List(l) => write!(f, "{}", l),
            Object::Lambda(l) => write!(f, "{}", l),
            Object::NativeFn(_) => f.write_str("<native_function>"),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Atom(_), Object::Atom(_)) => todo!(),
            (Object::Atom(_), Object::List(_)) => todo!(),
            (Object::Atom(_), Object::Lambda(_)) => todo!(),
            (Object::Atom(_), Object::NativeFn(_)) => todo!(),
            (Object::List(_), Object::Atom(_)) => todo!(),
            (Object::List(l1), Object::List(l2)) => l1 == l2,
            (Object::Lambda(l), Object::Lambda(m)) => l == m,
            (Object::NativeFn(f), Object::NativeFn(g)) => f == g,
            _ => false
        }
    }
}

impl Eq for Object {}

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
    Vec(Vec<Object>),
    HashMap(HashMap<Object, Object>),
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lit::Number(n) => write!(f, "{}", n),
            Lit::Bool(b) => write!(f, "{}", b),
            Lit::Str(s) => write!(f, "{}", s),
            Lit::Vec(v) => write!(f, "{}", v)
        }
    }
}

pub type NativeFn = fn(env: Rc<RefCell<Env>>, args: Vec<Object>) -> Result<Object, String>;