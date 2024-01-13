use lust_utils::{intern::InternedString, list::List, span::Span};
use num_rational::Rational64;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    sexprs: Vec<Sexpr>,
    span: Span,
}

impl Root {
    pub fn new(sexprs: Vec<Sexpr>, span: Span) -> Self {
        Self { sexprs, span }
    }

    pub fn sexprs(&self) -> &[Sexpr] {
        &self.sexprs
    }
}

impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.sexprs {
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sexpr {
    kind: Box<SexprKind>,
    span: Span,
}

impl Sexpr {
    pub fn new(kind: SexprKind, span: Span) -> Self {
        Self {
            kind: Box::new(kind),
            span,
        }
    }

    pub fn kind(&self) -> &SexprKind {
        &self.kind
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl Display for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SexprKind {
    Atom(Atom),
    List(SexprList),
    Vector(Vec<Sexpr>),
}

impl Display for SexprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SexprKind::Atom(a) => write!(f, "{}", a),
            SexprKind::List(l) => write!(f, "{}", l),
            SexprKind::Vector(v) => {
                write!(f, "[")?;
                for (i, s) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", s)?;
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SexprList {
    head: List<Sexpr>,
    span: Span,
}

impl SexprList {
    pub fn new(head: List<Sexpr>, span: Span) -> Self {
        Self { head, span }
    }

    pub fn head(&self) -> &List<Sexpr> {
        &self.head
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl Display for SexprList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, s) in self.head.iter().enumerate() {
            if i != 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", s)?;
        }
        write!(f, ")")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    kind: Box<AtomKind>,
    span: Span,
}

impl Atom {
    pub fn new(kind: AtomKind, span: Span) -> Self {
        Self {
            kind: Box::new(kind),
            span,
        }
    }

    pub fn kind(&self) -> &AtomKind {
        &self.kind
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AtomKind {
    Sym(InternedString),
    Lit(Lit),
}

impl Display for AtomKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtomKind::Sym(s) => write!(f, "{}", s),
            AtomKind::Lit(l) => write!(f, "{}", l),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Num(Rational64),
    Str(InternedString),
    Bool(bool),
    Char(char),
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lit::Num(n) => write!(f, "{}", n),
            Lit::Str(s) => write!(f, "{}", s),
            Lit::Bool(b) => write!(f, "{}", b),
            Lit::Char(c) => write!(f, "{}", c),
        }
    }
}
