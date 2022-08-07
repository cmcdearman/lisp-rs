use std::{rc::Rc, cell::RefCell, collections::HashMap};

use crate::{parser::ParseError, env::Env};

#[derive(Debug, Clone)]
pub enum Sexpr {
    Atom(Atom),
    Cons { car: Rc<RefCell<Sexpr>>, cdr: Rc<RefCell<Sexpr>> },
    Func(fn(Sexpr) -> Result<Sexpr, ParseError>),
    Env(Env),
    Nil
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Symbol(String),
    Literal(Literal)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String)
}



