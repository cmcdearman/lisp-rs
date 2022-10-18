pub enum Object {
    Atom(Atom),
    List(Vec<Object>),
    Fn(fn(Vec<Object>) -> Result<Object, String>),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Sym(String),
    Lit(Lit),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Num(f64),
    Str(String),
    Bool(bool),
}