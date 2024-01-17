use lust_utils::{
    intern::InternedString,
    list::List,
    num::{BigInt, BigRational, Float},
    span::Span,
};
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

    pub fn replace(&mut self, kind: SexprKind) {
        self.kind = Box::new(kind);
    }

    pub fn replace_sym(&mut self, sym: InternedString) {
        // recursively replace all instances of the symbol
        match self.kind() {
            SexprKind::Atom(a) => match a.kind() {
                AtomKind::Sym(s) => {
                    if s == &sym {
                        *self = Sexpr::new(
                            SexprKind::Atom(Atom::new(AtomKind::Sym(sym), *a.span())),
                            *self.span(),
                        );
                    }
                }
                _ => (),
            },
            SexprKind::SynList(l) => {
                for s in l.list().iter() {
                    s.replace_sym(sym.clone());
                }
            }
            SexprKind::DataList(l) => {
                for s in l.list().iter() {
                    s.replace_sym(sym.clone());
                }
            }
            SexprKind::Vector(v) => {
                for s in v {
                    s.replace_sym(sym.clone());
                }
            }
        }
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
    SynList(SynList),
    DataList(DataList),
    Vector(Vec<Sexpr>),
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
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SynList {
    list: List<Sexpr>,
    span: Span,
}

impl SynList {
    pub fn new(list: List<Sexpr>, span: Span) -> Self {
        Self { list, span }
    }

    pub fn list(&self) -> &List<Sexpr> {
        &self.list
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn head(&self) -> Option<&Sexpr> {
        self.list.head()
    }

    pub fn tail(&self) -> Option<&List<Sexpr>> {
        self.list.tail()
    }
}

impl Display for SynList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.list)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DataList {
    list: List<Sexpr>,
    span: Span,
}

impl DataList {
    pub fn new(list: List<Sexpr>, span: Span) -> Self {
        Self { list, span }
    }

    pub fn list(&self) -> &List<Sexpr> {
        &self.list
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn head(&self) -> Option<&Sexpr> {
        self.list.head()
    }

    pub fn tail(&self) -> Option<&List<Sexpr>> {
        self.list.tail()
    }
}

impl Display for DataList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, s) in self.list.iter().enumerate() {
            if i != 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", s)?;
        }
        write!(f, "]")
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
    Int(BigInt),
    Float(Float),
    Rational(BigRational),
    Str(InternedString),
    Bool(bool),
    Char(char),
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lit::Int(i) => write!(f, "{}", i),
            Lit::Float(fl) => write!(f, "{}", fl),
            Lit::Rational(r) => write!(f, "{}", r),
            Lit::Str(s) => write!(f, "{}", s),
            Lit::Bool(b) => write!(f, "{}", b),
            Lit::Char(c) => write!(f, "{}", c),
        }
    }
}
