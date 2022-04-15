enum Atom {
    Symbol,
    Literal
}

enum Sexpr {
  Atom(Atom),
  List(Vec<Sexpr>),
}

enum Literal {
  Number
}


