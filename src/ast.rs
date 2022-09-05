pub struct Ast {
    data: Vec<Sexpr>,
    pos: usize,
}

impl Ast {
    pub fn new() -> Self {
        Self { data: Vec::new(), pos: 0 }
    }

    pub fn push(&mut self, expr: Sexpr) -> usize {
        self.data.push(expr);
        self.pos = (self.data.len() - 1) as usize;
        self.pos
    }
}

impl Iterator for Ast {
    type Item = Sexpr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos <= 0 {
            return None;
        }
        let expr = &self.data[self.pos];
        self.pos -= 1;
        Some(*expr)
    }
}

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
    Str(String),
    Bool(bool)
}



