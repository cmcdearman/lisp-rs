#[derive(Debug, Clone)]
pub enum Sexpr {
    Atom(Atom),
    Cons(u32, u32),
    Nil
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Sym(String),
    Lit(Lit)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Num(f64),
    Str(String)
}



