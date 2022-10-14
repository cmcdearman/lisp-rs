// pub struct Ast {
//     data: std::vec::IntoIter<Sexpr>,
// }

// impl Ast {
//     pub fn new(data: Vec<Sexpr>) -> Self {
//         Self { data: data.into_iter() }
//     }
// }

// impl Iterator for Ast {
//     type Item = Sexpr;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.data.next_back()
//     }
// }

#[derive(Debug, Clone)]
pub enum Sexpr {
    Atom(Atom),
    Cons(u32, u32),
    Fn(fn(Vec<Sexpr>) -> Result<Sexpr, String>),
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
