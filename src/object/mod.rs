pub mod cons;
pub mod env;
pub mod lambda;
pub mod list;
pub mod number;
pub mod symbol;

use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use self::{list::List, lambda::Lambda, symbol::Symbol, number::Number, env::Env};


#[derive(Clone)]
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

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        todo!()
    }
}

impl Eq for Object {}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Lit {
    Num(Number),
    Bool(bool),
    Str(String),
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lit::Num(n) => write!(f, "{}", n),
            Lit::Bool(b) => write!(f, "{}", b),
            Lit::Str(s) => write!(f, "{}", s),
        }
    }
}

pub type NativeFn = fn(env: Rc<RefCell<Env>>, args: Vec<Object>) -> Result<Object, String>;

