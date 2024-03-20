use lust_utils::{
    intern::InternedString,
    list::List,
    num::{Int, Rational, Real},
    span::Span,
};
use std::{collections::HashMap, fmt::Display};

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

    pub fn span(&self) -> Span {
        self.span
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn as_special_form(&self) -> Option<&str> {
        match self.kind() {
            SexprKind::List(l) => match l.head() {
                Some(head) => match head.kind() {
                    SexprKind::Atom(a) => match a.kind() {
                        AtomKind::Sym(s) => match s.as_ref() {
                            "def" | "let" | "quote" | "fn" | "and" | "or" | "begin" => {
                                Some(s.as_ref())
                            }
                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                },
                None => None,
            },
            _ => None,
        }
    }

    pub fn as_atom(&self) -> Option<&Atom> {
        match self.kind() {
            SexprKind::Atom(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&List<Sexpr>> {
        match self.kind() {
            SexprKind::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn replace(&mut self, kind: SexprKind) {
        self.kind = Box::new(kind);
    }

    pub fn replace_sym(&mut self, sym: InternedString, arg: Sexpr) {
        // recursively replace all instances of the symbol
        match self.kind() {
            SexprKind::Atom(a) => match a.kind() {
                AtomKind::Sym(s) => {
                    if s == &sym {
                        *self = arg;
                    }
                }
                _ => (),
            },
            SexprKind::List(l) => {
                let mut new_vec = vec![];
                for s in l.list().iter() {
                    let mut new_s = s.clone();
                    new_s.replace_sym(sym.clone(), arg.clone());
                    new_vec.push(new_s);
                }
                let new_list = List::from(new_vec);
                *self = Sexpr::new(SexprKind::List(new_list), self.span());
            }
        }
    }
}

impl Display for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Eq for Sexpr {}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SexprKind {
    Atom(Atom),
    List(List<Sexpr>),
}

impl Display for SexprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SexprKind::Atom(a) => write!(f, "{}", a),
            SexprKind::SynList(l) => write!(f, "{}", l),
            SexprKind::DataList(l) => write!(f, "{}", l),
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
            SexprKind::Table(t) => {
                write!(f, "{{")?;
                for (i, (k, v)) in t.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

    pub fn as_lit(&self) -> Option<&Lit> {
        match self.kind() {
            AtomKind::Lit(l) => Some(l),
            _ => None,
        }
    }

    pub fn as_sym(&self) -> Option<&InternedString> {
        match self.kind() {
            AtomKind::Sym(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_path(&self) -> Option<&Vec<InternedString>> {
        match self.kind() {
            AtomKind::Path(p) => Some(p),
            _ => None,
        }
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum AtomKind {
    Lit(Lit),
    Sym(InternedString),
    Path(Vec<InternedString>),
}

impl Display for AtomKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtomKind::Lit(l) => write!(f, "{}", l),
            AtomKind::Sym(s) => write!(f, "{}", s),
            AtomKind::Path(p) => write!(f, "{}", p.join(".")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Lit {
    Int(Int),
    BigInt(BigInt),
    Real(Real),
    Rational(Rational),
    BigRatio(BigRational),
    String(InternedString),
    Bool(bool),
    Char(char),
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lit::Int(i) => write!(f, "{}", i),
            Lit::BigInt(i) => write!(f, "{}", i),
            Lit::Real(r) => write!(f, "{}", r),
            Lit::Rational(r) => write!(f, "{}", r),
            Lit::BigRatio(r) => write!(f, "{}", r),
            Lit::String(s) => write!(f, "{}", s),
            Lit::Bool(b) => write!(f, "{}", b),
            Lit::Char(c) => write!(f, "{}", c),
        }
    }
}
