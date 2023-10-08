use crate::util::{
    format::{spaces, Format},
    intern::InternedString,
    node::SrcNode,
};
use num_rational::Rational64;
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    pub sexprs: Vec<Sexpr>,
}

// impl Display for Root {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for s in self.clone().sexprs {
//             writeln!(f, "{}", s)?;
//         }
//         Ok(())
//     }
// }

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
    Pair(SrcNode<Pair>),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pair {
    head: Sexpr,
    tail: Sexpr,
}

impl Pair {
    pub fn new(head: Sexpr, tail: Sexpr) -> Self {
        Self { head, tail }
    }

    pub fn head(&self) -> Sexpr {
        self.head.clone()
    }

    pub fn tail(&self) -> Sexpr {
        self.tail.clone()
    }
}

impl IntoIterator for SrcNode<Pair> {
    type Item = Sexpr;
    type IntoIter = PairIter;

    fn into_iter(self) -> Self::IntoIter {
        PairIter(self.clone())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PairIter(SrcNode<Pair>);

impl FromIterator<Sexpr> for PairIter {
    fn from_iter<T: IntoIterator<Item = Sexpr>>(iter: T) -> Self {
        todo!()
    }
}

impl Iterator for PairIter {
    type Item = Sexpr;

    fn next(&mut self) -> Option<Self::Item> {
        let head = self.0.inner().head();
        match self.0.tail() {
            Sexpr::Pair(pair) => self.0 = pair,
            Sexpr::Nil => return None,
            tail => self.0.inner().head = tail,
        };
        Some(head)
    }
}

// // '(1 . 2)

// impl FromIterator<Sexpr> for ConsIter {
//     fn from_iter<T: IntoIterator<Item = Sexpr>>(iter: T) -> Self {
//         ConsIter(iter.into_iter().fold(None, |acc, next| {
//             Some(Cons {
//                 head: Rc::new(RefCell::new(next)),
//                 tail: {
//                     if let Some(last) = acc {
//                         Some(Rc::new(RefCell::new(SrcNode::new(
//                             Sexpr::Cons(last),
//                             next.span(),
//                         ))))
//                     } else {
//                         None
//                     }
//                 },
//             })
//         }))
//     }
// }

// impl Iterator for ConsIter {
//     type Item = Sexpr;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(tail) = self.0 {
//             self.0 = SrcNode::new(Rc::new(RefCell::new(tail.inner().clone())), tail.span());
//             Some(self.0.head())
//         } else {
//             None
//         }
//     }
// }

// impl DoubleEndedIterator for ConsIter {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

// impl ExactSizeIterator for ConsIter {
//     fn len(&self) -> usize {
//         let mut len = 0;
//         let mut iter = self.clone();
//         while iter.next().is_some() {
//             len += 1;
//         }
//         len
//     }
// }

// impl Container<Sexpr> for ConsIter {
//     fn push(&mut self, item: Sexpr) {
//         todo!()
//     }
// }

// impl Display for Sexpr {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self.inner().clone() {
//             Sexpr::Atom(a) => write!(f, "{}", a.clone()),
//             Sexpr::Cons(c) => {
//                 write!(f, "(")?;
//                 for (i, s) in c.clone().into_iter().enumerate() {
//                     if i > 0 {
//                         write!(f, " ")?;
//                     }
//                     write!(f, "{}", s)?;
//                 }
//                 write!(f, ")")
//             }
//         }
//     }
// }

// impl Debug for Format<Sexpr> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Pretty print with indents and spans
//         match self.value.inner().clone() {
//             Sexpr::Atom(a) => {
//                 let fmt = Format::new(self.indent + 2, a.clone());
//                 write!(
//                     f,
//                     "{}Atom @ {}\n{:?} @ {}",
//                     spaces(self.indent),
//                     self.value.span(),
//                     fmt,
//                     self.value.span()
//                 )
//             }
//             Sexpr::Cons(cons) => {
//                 write!(f, "{}Cons @ {}", spaces(self.indent), self.value.span())?;
//                 let iter = cons.clone().into_iter();
//                 for (i, sexpr) in iter.clone().enumerate() {
//                     write!(f, "\n{:?}", Format::new(self.indent + 2, sexpr))?;
//                     if i != iter.len() - 1 {
//                         write!(f, ",")?;
//                     }
//                 }
//                 Ok(())
//             }
//         }
//     }
// }

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

// impl Debug for Format<Atom> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Pretty print with indents and spans
//         match self.value {
//             Atom::Symbol(name) => {
//                 write!(f, "{}Symbol({})", spaces(self.indent), name,)
//             }
//             Atom::Number(n) => {
//                 write!(f, "{}Number({})", spaces(self.indent), n,)
//             }
//             Atom::String(s) => {
//                 write!(f, "{}String({})", spaces(self.indent), s,)
//             }
//         }
//     }
// }

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
