use lust_util::{intern::InternedString, list::List, span::Spanned};
use num_rational::Rational64;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    pub sexprs: Vec<Spanned<Sexpr>>,
}

impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.clone().sexprs {
            writeln!(f, "{}", s.value)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sexpr {
    Atom(Spanned<Atom>),
    Cons(List<Spanned<Sexpr>>),
}

impl Display for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.clone() {
            Sexpr::Atom(a) => write!(f, "{}", a.value),
            Sexpr::Cons(l) => {
                write!(f, "(")?;
                for (i, s) in l.enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", s.value)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Lit(Lit),
    Symbol(InternedString),
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Lit(l) => write!(f, "{}", l),
            Atom::Symbol(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(i64),
    Rational(Rational64),
    Real(f64),
    Char(char),
    String(InternedString),
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lit::Int(i) => write!(f, "{}", i),
            Lit::Rational(r) => write!(f, "{}", r),
            Lit::Real(r) => write!(f, "{}", r),
            Lit::Char(c) => write!(f, "#\\{}", c),
            Lit::String(s) => write!(f, "{:?}", s),
        }
    }
}

// Sexpr format derive
// (+ 1 2)
// Root 0..7
//   Cons 0..7
//     Atom 0..1
//       Symbol 0..1
//         "+"
//     Atom 2..3
//       Lit 2..3
//         Int 2..3
//           1
//     Atom 4..5
//       Lit 4..5
//         Int 4..5
//           2
