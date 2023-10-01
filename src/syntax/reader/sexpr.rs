use crate::util::{
    format::{spaces, Format},
    intern::InternedString,
    node::SrcNode,
};
use num_rational::Rational64;
use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq)]
pub struct Root {
    pub sexprs: Vec<SrcNode<Sexpr>>,
}

impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.clone().sexprs {
            writeln!(f, "{}", s.inner())?;
        }
        Ok(())
    }
}

impl Debug for SrcNode<Root> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty print with indents and spans
        write!(f, "Root @ {}\n", self.span())?;
        for sexpr in self.sexprs.clone() {
            write!(f, "{:?}", Format::new(2, sexpr))?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sexpr {
    Atom(SrcNode<Atom>),
    Cons(Cons),
}

impl Sexpr {
    pub fn is_nil(&self) -> bool {
        match self.clone() {
            Sexpr::Atom(a) => match a.inner().clone() {
                Atom::Symbol(s) => &*s == "nil",
                _ => false,
            },
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cons {
    car: SrcNode<Sexpr>,
    cdr: SrcNode<Sexpr>,
}

impl Cons {
    pub fn new(car: SrcNode<Sexpr>, cdr: SrcNode<Sexpr>) -> Self {
        Self { car, cdr }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsIter {
    pub cons: Cons,
}

impl Iterator for ConsIter {
    type Item = SrcNode<Sexpr>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cons.cdr.is_nil() {
            None
        } else {
            let car = self.cons.car.clone();
            let cdr = self.cons.cdr.clone();
            self.cons = cdr.inner().clone().into_cons().unwrap();
            Some(car)
        }
    }
}

impl Display for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.clone() {
            Sexpr::Atom(a) => write!(f, "{}", a.inner()),
            Sexpr::Cons(l) => {
                write!(f, "(")?;
                for (i, s) in l.enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", s.inner())?;
                }
                write!(f, ")")
            }
        }
    }
}

impl Debug for Format<SrcNode<Sexpr>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty print with indents and spans
        match &self.value.inner() {
            Sexpr::Atom(a) => {
                let fmt = Format::new(self.indent + 2, a.clone());
                write!(
                    f,
                    "{}Atom @ {}\n{:?}",
                    spaces(self.indent),
                    self.value.span(),
                    fmt,
                )
            }
            Sexpr::Cons(list) => {
                write!(f, "{}List @ {}", spaces(self.indent), self.value.span())?;
                for (i, sexpr) in list.clone().into_iter().rev().enumerate() {
                    write!(f, "\n{:?}", Format::new(self.indent + 2, sexpr))?;
                    if i != list.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl Default for Sexpr {
    fn default() -> Self {
        Self::Atom(Atom::Nil)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Symbol(InternedString),
    Number(Rational64),
    String(InternedString),
    Nil,
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Symbol(s) => write!(f, "{}", s),
            Atom::Number(n) => write!(f, "{}", n),
            Atom::String(s) => write!(f, "{}", s),
            Atom::Nil => write!(f, "nil"),
        }
    }
}

impl Debug for Format<SrcNode<Atom>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty print with indents and spans
        match &self.value.inner() {
            Atom::Symbol(name) => {
                write!(
                    f,
                    "{}Symbol({}) @ {}",
                    spaces(self.indent),
                    name,
                    self.value.span()
                )
            }
            Atom::Number(n) => {
                write!(
                    f,
                    "{}Number({}) @ {}",
                    spaces(self.indent),
                    n,
                    self.value.span()
                )
            }
            Atom::String(s) => {
                write!(
                    f,
                    "{}String({}) @ {}",
                    spaces(self.indent),
                    s,
                    self.value.span()
                )
            }
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
