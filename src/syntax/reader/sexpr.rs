use crate::util::{
    format::{spaces, Format},
    intern::InternedString,
    node::SrcNode,
    span::Span,
};
use num_rational::Rational64;
use std::{
    cmp::max,
    fmt::{Debug, Display},
};

#[derive(Clone, PartialEq)]
pub struct Root(pub Vec<Sexpr>);

impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.clone().0 {
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

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

#[derive(Debug, Clone, PartialEq)]
pub enum Sexpr {
    Atom(SrcNode<Atom>),
    Pair(SrcNode<Pair>),
    Nil,
}

impl Sexpr {
    pub fn span(&self) -> Span {
        match self {
            Sexpr::Atom(a) => a.span(),
            Sexpr::Pair(p) => p.span(),
            Sexpr::Nil => Span::default(),
        }
    }
}

impl Display for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.clone() {
            Sexpr::Atom(a) => write!(f, "{}", a.inner().clone()),
            Sexpr::Pair(_) => {
                write!(f, "(")?;
                for (i, s) in self.clone().into_iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", s)?;
                }
                write!(f, ")")
            }
            Sexpr::Nil => write!(f, "()"),
        }
    }
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

impl FromIterator<Sexpr> for Sexpr {
    fn from_iter<T: IntoIterator<Item = Sexpr>>(iter: T) -> Self {
        iter.into_iter().fold(Sexpr::Nil, |acc, next| {
            Sexpr::Pair(SrcNode::new(
                Pair::new(next.clone(), acc.clone()),
                // acc.span().extend(sexpr.span()),
                Span::new(next.span().start, max(acc.span().end, next.span().end)),
            ))
        })
    }
}

impl IntoIterator for Sexpr {
    type Item = Sexpr;
    type IntoIter = PairIter;

    fn into_iter(self) -> Self::IntoIter {
        PairIter(self.clone())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PairIter(Sexpr);

impl Iterator for PairIter {
    type Item = Sexpr;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.clone() {
            Sexpr::Pair(pair) => {
                self.0 = pair.tail();
                Some(pair.head())
            }
            Sexpr::Nil => None,
            sexpr => {
                self.0 = Sexpr::Nil;
                Some(sexpr)
            }
        }
    }
}

impl ExactSizeIterator for PairIter {
    fn len(&self) -> usize {
        self.clone().fold(0, |acc, _| acc + 1)
    }
}

impl Debug for Format<Sexpr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty print with indents and spans
        match self.value.clone() {
            Sexpr::Atom(a) => {
                let fmt = Format::new(self.indent + 2, a.inner().clone());
                write!(
                    f,
                    "{}Atom @ {}\n{:?}",
                    spaces(self.indent),
                    self.value.span(),
                    fmt,
                )
            }
            Sexpr::Pair(p) => {
                write!(f, "{}Pair @ {}", spaces(self.indent), p.span())?;
                write!(
                    f,
                    "\n{}head:\n{:?}",
                    spaces(self.indent + 2),
                    Format::new(self.indent + 4, p.head())
                )?;
                write!(
                    f,
                    "\n{}tail:\n{:?}",
                    spaces(self.indent + 2),
                    Format::new(self.indent + 4, p.tail())
                )?;
                Ok(())
            }
            Sexpr::Nil => write!(f, "{}Nil", spaces(self.indent)),
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
