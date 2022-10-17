#[derive(Debug, Clone)]
pub enum Sexpr {
    Atom(Atom),
    List(Vec<Sexpr>),
    // Fn(fn(Vec<Sexpr>) -> Result<Sexpr, String>),
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
