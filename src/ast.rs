use std::fmt::{Debug, format, Formatter};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum Sexpr {
    Atom(Atom),
    Cons { car: Box<Sexpr>, cdr: Box<Sexpr> },
    Nil
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Atom {
    Symbol(String),
    Literal(Literal)
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String)
}

// impl Debug for Sexpr {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             serde_json::to_string_pretty(self).unwrap()
//         )
//     }
// }
