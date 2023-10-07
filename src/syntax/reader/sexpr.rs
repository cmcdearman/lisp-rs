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
    Atom(Atom),
    List(List),
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum Cons {

// }

#[derive(Debug, Clone, PartialEq)]
pub enum List {
    Pair {
        head: Rc<RefCell<SrcNode<Sexpr>>>,
        tail: Option<Rc<RefCell<SrcNode<Sexpr>>>>,
    },
    Nil,
}

// impl Cons {
//     pub fn new(head: SrcNode<Sexpr>, tail: SrcNode<Sexpr>) -> Self {
//         Self {
//             head: Rc::new(RefCell::new(head.clone())),
//             tail: Some(Rc::new(RefCell::new(tail.clone()))),
//         }
//     }

//     pub fn head(&self) -> SrcNode<Sexpr> {
//         self.head.borrow().clone()
//     }

//     pub fn tail(&self) -> Option<SrcNode<Sexpr>> {
//         if let Some(tail) = self.tail {
//             Some(tail.borrow().clone())
//         } else {
//             None
//         }
//     }

//     pub fn set_head(&mut self) {}
// }

// impl IntoIterator for Cons {
//     type Item = SrcNode<Sexpr>;
//     type IntoIter = ConsIter;

//     fn into_iter(self) -> Self::IntoIter {
//         ConsIter(Some(self.clone()))
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct ConsIter(Option<Cons>);

// // '(1 . 2)

// impl FromIterator<SrcNode<Sexpr>> for ConsIter {
//     fn from_iter<T: IntoIterator<Item = SrcNode<Sexpr>>>(iter: T) -> Self {
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
//     type Item = SrcNode<Sexpr>;

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

// impl Container<SrcNode<Sexpr>> for ConsIter {
//     fn push(&mut self, item: SrcNode<Sexpr>) {
//         todo!()
//     }
// }

impl Display for SrcNode<Sexpr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.inner().clone() {
            Sexpr::Atom(a) => write!(f, "{}", a.clone()),
            Sexpr::Cons(c) => {
                write!(f, "(")?;
                for (i, s) in c.clone().into_iter().enumerate() {
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
                    "{}Atom @ {}\n{:?} @ {}",
                    spaces(self.indent),
                    self.value.span(),
                    fmt,
                    self.value.span()
                )
            }
            Sexpr::Cons(cons) => {
                write!(f, "{}Cons @ {}", spaces(self.indent), self.value.span())?;
                let iter = cons.clone().into_iter();
                for (i, sexpr) in iter.clone().enumerate() {
                    write!(f, "\n{:?}", Format::new(self.indent + 2, sexpr))?;
                    if i != iter.len() - 1 {
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

impl Debug for Format<Atom> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty print with indents and spans
        match self.value {
            Atom::Symbol(name) => {
                write!(f, "{}Symbol({})", spaces(self.indent), name,)
            }
            Atom::Number(n) => {
                write!(f, "{}Number({})", spaces(self.indent), n,)
            }
            Atom::String(s) => {
                write!(f, "{}String({})", spaces(self.indent), s,)
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
