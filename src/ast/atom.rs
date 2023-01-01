use super::{symbol::Symbol, number::Number};

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Sym(Symbol),
    Num(Number),
    Bool(bool),
    Str(String)
}