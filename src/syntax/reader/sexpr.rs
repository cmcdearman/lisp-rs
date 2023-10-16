use crate::util::{
    format::{spaces, Format},
    intern::InternedString,
    node::SrcNode,
};
use num_rational::Rational64;
use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq)]
pub struct Root(pub Vec<SrcNode<Sexpr>>);

// impl Display for Root {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for s in self.clone().0 {
//             writeln!(f, "{}", s)?;
//         }
//         Ok(())
//     }
// }

impl Debug for SrcNode<Root> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty print with indents and spans
        write!(f, "Root @ {}\n", self.span())?;
        for sexpr in self.0.clone() {
            write!(f, "{:?}", Format::new(2, sexpr))?;
        }
        Ok(())
    }
}

impl IntoIterator for Root {
    type Item = SrcNode<Sexpr>;
    type IntoIter = std::vec::IntoIter<SrcNode<Sexpr>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sexpr {
    Atom(Atom),
    Pair(SrcNode<Sexpr>, SrcNode<Sexpr>),
    List(Vec<SrcNode<Sexpr>>),
}

// impl Display for Sexpr {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self.clone() {
//             Sexpr::Atom(a) => write!(f, "{}", a.inner().clone()),
//             Sexpr::Pair(_) => {
//                 write!(f, "(")?;
//                 for (i, s) in self.clone().into_iter().enumerate() {
//                     if i > 0 {
//                         write!(f, " ")?;
//                     }
//                     write!(f, "{}", s)?;
//                 }
//                 write!(f, ")")
//             }
//             Sexpr::Nil => write!(f, "()"),
//         }
//     }
// }

impl Debug for Format<SrcNode<Sexpr>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty print with indents and spans
        match self.value.inner().clone() {
            Sexpr::Atom(a) => {
                let fmt = Format::new(self.indent + 2, a.clone());
                write!(
                    f,
                    "{}Atom @ {}\n{:?}\n",
                    spaces(self.indent),
                    self.value.span(),
                    fmt,
                )
            }
            Sexpr::Pair(head, tail) => {
                write!(
                    f,
                    "{}Pair @ {}\n{:?}\n{:?}",
                    spaces(self.indent),
                    self.value.span(),
                    Format::new(self.indent + 2, head.clone()),
                    Format::new(self.indent + 2, tail.clone()),
                )
            }
            Sexpr::List(l) => {
                let fmt = Format::new(self.indent + 4, l.clone());
                write!(
                    f,
                    "{}List @ {}\n{:?}\n",
                    spaces(self.indent),
                    self.value.span(),
                    fmt,
                )
            }
        }
    }
}

impl Debug for Format<Vec<SrcNode<Sexpr>>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty print with indents and spans
        // write!(f, "{}List @ {}\n", spaces(self.indent), self.value.span())?;
        for sexpr in self.value.clone() {
            write!(f, "{:?}", Format::new(self.indent, sexpr))?;
        }
        Ok(())
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
        match self.value.clone() {
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
