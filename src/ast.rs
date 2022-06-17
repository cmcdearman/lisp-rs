use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Sexpr {
    Atom(Atom),
    Cons((Rc<Sexpr>, Rc<Sexpr>)),
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
