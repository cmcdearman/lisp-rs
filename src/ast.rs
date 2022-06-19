use std::rc::Rc;

use crate::parser::ParseError;

#[derive(Debug, Clone)]
pub enum Sexpr {
    Atom(Atom),
    Cons { car: Rc<Sexpr>, cdr: Rc<Sexpr> },
    Func(fn(Sexpr) -> Result<Sexpr, ParseError>),
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



