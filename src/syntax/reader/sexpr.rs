use crate::util::{
    format::{spaces, Format},
    intern::InternedString,
    meta::Meta,
    span::Span,
};
use num_rational::Rational64;
use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq)]
pub struct Root {
    pub sexprs: Vec<Sexpr>,
    pub meta: Meta,
}

// impl Display for Root {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for s in self.clone().0 {
//             writeln!(f, "{}", s)?;
//         }
//         Ok(())
//     }
// }

impl Debug for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty print with indents and spans
        write!(f, "Root @ {}\n", self.meta.span)?;
        for sexpr in self.sexprs.clone() {
            write!(f, "{:?}", Format::new(2, sexpr))?;
        }
        Ok(())
    }
}

impl IntoIterator for Root {
    type Item = Sexpr;
    type IntoIter = std::vec::IntoIter<Sexpr>;

    fn into_iter(self) -> Self::IntoIter {
        self.sexprs.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sexpr {
    Atom {
        value: Atom,
        meta: Meta,
    },
    Pair {
        head: Box<Self>,
        tail: Box<Self>,
        meta: Meta,
    },
    List {
        values: Vec<Self>,
        meta: Meta,
    },
    Vector {
        values: Vec<Self>,
        meta: Meta,
    },
    ByteVector {
        values: Vec<u8>,
        meta: Meta,
    },
}

impl Sexpr {
    pub fn span(&self) -> Span {
        match self {
            Sexpr::Atom { meta, .. } => meta.span,
            Sexpr::Pair { meta, .. } => meta.span,
            Sexpr::List { meta, .. } => meta.span,
            Sexpr::Vector { values, meta } => meta.span,
            Sexpr::ByteVector { values, meta } => meta.span,
        }
    }
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

impl Debug for Format<Sexpr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty print with indents and spans
        match self.value.clone() {
            Sexpr::Atom { value, meta } => {
                let fmt = Format::new(self.indent + 2, value.clone());
                write!(
                    f,
                    "{}Atom @ {}\n{:?}\n",
                    spaces(self.indent),
                    meta.span,
                    fmt,
                )
            }
            Sexpr::Pair { head, tail, meta } => {
                write!(
                    f,
                    "{}Pair @ {}\n{:?}{:?}",
                    spaces(self.indent),
                    meta.span,
                    Format::new(self.indent + 2, *head.clone()),
                    Format::new(self.indent + 2, *tail.clone()),
                )
            }
            Sexpr::List { values, meta } => {
                let fmt = Format::new(self.indent + 2, values.clone());
                write!(f, "{}List @ {}\n{:?}", spaces(self.indent), meta.span, fmt,)
            }
            Sexpr::Vector { values, meta } => {
                let fmt = Format::new(self.indent + 2, values.clone());
                write!(
                    f,
                    "{}Vector @ {}\n{:?}",
                    spaces(self.indent),
                    meta.span,
                    fmt,
                )
            }
            Sexpr::ByteVector { values, meta } => {
                let fmt = Format::new(self.indent + 2, values.clone());
                write!(
                    f,
                    "{}ByteVector @ {}\n{:?}",
                    spaces(self.indent),
                    meta.span,
                    fmt,
                )
            }
        }
    }
}

impl Debug for Format<Vec<Sexpr>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty print with indents and spans
        for sexpr in self.value.clone() {
            write!(f, "{:?}", Format::new(self.indent, sexpr))?;
        }
        Ok(())
    }
}

impl Debug for Format<Vec<u8>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty print with indents and spans
        for byte in self.value.clone() {
            write!(f, "{:?}", byte)?;
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
