use crate::util::{
    format::{spaces, Format},
    intern::InternedString,
    node::SrcNode,
};
use num_rational::Rational64;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    pub sexprs: Vec<SrcNode<Sexpr>>,
}

impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.clone().sexprs {
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

// impl Debug for SrcNode<Root> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Pretty print with indents and spans
//         write!(f, "Root @ {}\n", self.span())?;
//         for sexpr in self.sexprs.clone() {
//             write!(f, "{:?}", Format::new(2, sexpr))?;
//         }
//         Ok(())
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub enum Sexpr {
    Atom(SrcNode<Atom>),
    Cons(SrcNode<Cons>),
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

    pub fn is_atom(&self) -> bool {
        match self {
            Sexpr::Atom(_) => true,
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

// impl FromIterator<SrcNode<Sexpr>> for Cons {
//     fn from_iter<T: IntoIterator<Item = SrcNode<Sexpr>>>(iter: T) -> Self {
//         let mut iter = iter.into_iter();
//         match iter.next() {
//             Some(car) => Self::from_iter_with_car(car, iter),
//             None => Self::from_iter_with_car(
//                 SrcNode::new(Sexpr::Atom(SrcNode::new(
//                     Atom::Symbol(InternedString::from("nil")),
//                     Span::default(),
//                 ))),
//                 iter,
//             ),
//         }
//         // let cdr = iter.next().unwrap();
//         // Self { car, cdr }
//     }
// }

impl IntoIterator for SrcNode<Sexpr> {
    type Item = SrcNode<Sexpr>;
    type IntoIter = ConsIter;

    fn into_iter(self) -> Self::IntoIter {
        ConsIter {
            next: Some(self.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsIter {
    next: Option<SrcNode<Sexpr>>,
}

// '(1 . 2)

impl Iterator for ConsIter {
    type Item = SrcNode<Sexpr>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(sexpr) => match sexpr.inner() {
                Sexpr::Cons(c) => {
                    self.next = Some(c.cdr.clone());
                    Some(c.car.clone())
                }
                _ => None,
            },
            None => None,
        }
    }
}

impl DoubleEndedIterator for ConsIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(sexpr) => match sexpr.inner() {
                Sexpr::Cons(c) => {
                    self.next = Some(c.car.clone());
                    Some(c.cdr.clone())
                }
                _ => None,
            },
            None => None,
        }
    }
}

impl ExactSizeIterator for ConsIter {
    fn len(&self) -> usize {
        let mut len = 0;
        let mut iter = self.clone();
        while iter.next().is_some() {
            len += 1;
        }
        len
    }
}

impl Display for SrcNode<Sexpr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.inner().clone() {
            Sexpr::Atom(a) => write!(f, "{}", a.inner()),
            Sexpr::Cons(c) => {
                write!(f, "(")?;
                for (i, s) in self.clone().into_iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", s)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl Debug for Format<SrcNode<Sexpr>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty print with indents and spans
        match self.value.inner().clone() {
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
            Sexpr::Cons(cons) => {
                write!(f, "{}List @ {}", spaces(self.indent), self.value.span())?;
                for (i, sexpr) in self.value.clone().into_iter().rev().enumerate() {
                    write!(f, "\n{:?}", Format::new(self.indent + 2, sexpr))?;
                    if i != self.value.clone().into_iter().len() - 1 {
                        write!(f, ",")?;
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Symbol(InternedString),
    Number(Rational64),
    String(InternedString),
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Symbol(s) => write!(f, "{}", s),
            Atom::Number(n) => write!(f, "{}", n),
            Atom::String(s) => write!(f, "{}", s),
        }
    }
}

impl Debug for Format<SrcNode<Atom>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty print with indents and spans
        match self.value.inner() {
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
