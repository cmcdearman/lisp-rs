use lust_utils::{
    intern::InternedString,
    list::List,
    num::{BigInt, BigRational, Int, Rational, Real},
    span::Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    decls: Vec<Decl>,
    span: Span,
}

impl Root {
    pub fn new(decls: Vec<Decl>, span: Span) -> Self {
        Self { decls, span }
    }

    pub fn decls(&self) -> &[Decl] {
        &self.decls
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Decl {
    kind: Box<DeclKind>,
    span: Span,
}

impl Decl {
    pub fn new(kind: DeclKind, span: Span) -> Self {
        Self {
            kind: Box::new(kind),
            span,
        }
    }

    pub fn kind(&self) -> &DeclKind {
        &self.kind
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclKind {
    Def { name: InternedString, expr: Expr },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    kind: Box<ExprKind>,
    span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self {
            kind: Box::new(kind),
            span,
        }
    }

    pub fn kind(&self) -> &ExprKind {
        &self.kind
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Lit(Lit),
    Ident(InternedString),
    Let {
        name: InternedString,
        expr: Expr,
        body: Expr,
    },
    // Match {
    //     expr: Expr,
    //     arms: Vec<MatchArm>,
    //     span: Span,
    // },
    If {
        cond: Expr,
        then: Expr,
        else_: Expr,
    },
    Lambda {
        param: InternedString,
        body: Expr,
    },
    List(List<Expr>),
    // Vector(Vec<Expr>),
    // Map(BTreeMap<Expr, Expr>),
    // MapAccess {
    //     map: Expr,
    //     key: Expr,
    //     span: Span,
    // },
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct MatchArm {
//     pat: Pattern,
//     expr: Expr,
//     span: Span,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Pattern {
//     kind: Box<PatternKind>,
//     span: Span,
// }

// impl Pattern {
//     pub fn new(kind: PatternKind, span: Span) -> Self {
//         Self {
//             kind: Box::new(kind),
//             span,
//         }
//     }

//     pub fn kind(&self) -> &PatternKind {
//         &self.kind
//     }

//     pub fn span(&self) -> Span {
//         self.span
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum PatternKind {
//     Lit(Lit),
//     Ident(InternedString),
//     List(List<Pattern>),
// }

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(Int),
    BigInt(BigInt),
    Real(Real),
    Rational(Rational),
    BigRational(BigRational),
    String(InternedString),
    Bool(bool),
    Char(char),
}
