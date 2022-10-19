#[derive(Clone)]
pub enum Sexpr {
    Atom(Atom),
    List(Vec<Sexpr>),
    Fn(fn(&[Sexpr]) -> Result<Sexpr, String>),
}

impl std::fmt::Display for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Sexpr::Atom(a) => {
                match a {
                    Atom::Sym(s) => s.clone(),
                    Atom::Lit(l) => {
                        match l {
                            Lit::Num(n) => n.to_string(),
                            Lit::Str(s) => s.to_string(),
                            Lit::Bool(b) => b.to_string(),
                        }
                    }
                }
            }
            Sexpr::List(list) => {
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(","))
            }
            Sexpr::Fn(_) => "Function {}".to_string(),
        };

        write!(f, "{}", str)
    }
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
