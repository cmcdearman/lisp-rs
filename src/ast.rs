#[derive(Debug)]
pub enum Sexpr {
    Atom(Atom),
    List(Vec<Sexpr>)
}

// impl Sexpr {
//     pub fn to_string(&self) -> String {
//         s
//     }
// }

#[derive(Debug)]
pub enum Atom {
    Symbol(String),
    Literal(Literal)
}

#[derive(Debug)]
pub enum Literal {
  Number(i32)
}


