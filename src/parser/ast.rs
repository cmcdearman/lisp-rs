#[derive(Debug, Clone, PartialEq)]
pub enum Sexpr {
    Atom(Atom),
    List(Vec<Sexpr>)
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


