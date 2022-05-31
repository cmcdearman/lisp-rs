use std::collections::VecDeque;

pub type Ast = VecDeque<Sexpr>;

#[derive(Debug, Clone, PartialEq)]
pub enum Sexpr {
    Atom(Atom),
    List(VecDeque<Sexpr>)
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
