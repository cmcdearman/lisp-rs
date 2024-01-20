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

    pub fn span(&self) -> &Span {
        &self.span
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
            SexprKind::SynList(l) => {
                let mut new_vec = vec![];
                for s in l.list().iter() {
                    let mut new_s = s.clone();
                    new_s.replace_sym(sym.clone(), arg.clone());
                    new_vec.push(new_s);
                }
                let new_list = List::from(new_vec);
                *self = Sexpr::new(
                    SexprKind::SynList(SynList::new(new_list, *l.span())),
                    *self.span(),
                );
            }
            SexprKind::DataList(l) => {
                let mut new_vec = vec![];
                for s in l.list().iter() {
                    let mut new_s = s.clone();
                    new_s.replace_sym(sym.clone(), arg.clone());
                    new_vec.push(new_s);
                }
                let new_list = List::from(new_vec);
                *self = Sexpr::new(
                    SexprKind::DataList(DataList::new(new_list, *l.span())),
                    *self.span(),
                );
            }
            SexprKind::Vector(v) => {
                let mut new_vec = vec![];
                for s in v.iter() {
                    let mut new_s = s.clone();
                    new_s.replace_sym(sym.clone(), arg.clone());
                    new_vec.push(new_s);
                }
                *self = Sexpr::new(SexprKind::Vector(new_vec), *self.span());
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
