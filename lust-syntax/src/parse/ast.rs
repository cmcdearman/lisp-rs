use lust_utils::{
    intern::InternedString,
    list::List,
    num::{BigInt, BigRational, Int, Rational, Real},
    span::Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    pub defs: Vec<Def>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Def(pub Pattern, pub Expr);

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: Box<ExprKind>,
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self {
            kind: Box::new(kind),
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Lit(Lit),
    Ident(InternedString),
    Let(Pattern, Expr, Expr),
    Match(Expr, Vec<MatchArm>),
    Lambda(Pattern, Expr),
    List(List<Expr>),
    Vector(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pat: Pattern,
    expr: Expr,
    span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub kind: Box<PatternKind>,
    pub span: Span,
}

impl Pattern {
    pub fn new(kind: PatternKind, span: Span) -> Self {
        Self {
            kind: Box::new(kind),
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternKind {
    Lit(Lit),
    Ident(InternedString),
    List(List<Pattern>),
}

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
