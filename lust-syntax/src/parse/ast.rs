use std::{collections::HashMap, hash::Hash};

use lust_utils::{
    intern::InternedString,
    list::List,
    num::{BigInt, BigRational, Float},
    span::Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    items: Vec<Item>,
    span: Span,
}

impl Root {
    pub fn new(items: Vec<Item>, span: Span) -> Self {
        Self { items, span }
    }

    pub fn items(&self) -> &[Item] {
        &self.items
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    kind: Box<ItemKind>,
    span: Span,
}

impl Item {
    pub fn new(kind: ItemKind, span: Span) -> Self {
        Self {
            kind: Box::new(kind),
            span,
        }
    }

    pub fn kind(&self) -> &ItemKind {
        &self.kind
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemKind {
    Decl(Decl),
    Expr(Expr),
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
    Def {
        name: InternedString,
        expr: Expr,
        span: Span,
    },
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
        expr: Box<Expr>,
        body: Box<Expr>,
        span: Span,
    },
    List(List<Expr>),
    Vector(Vec<Expr>),
    // Table(),
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct Table {
//     rows: Vec<Row>,
//     span: Span,
// }

// impl Table {
//     pub fn new(rows: Vec<Row>, span: Span) -> Self {
//         Self { rows, span }
//     }

//     pub fn rows(&self) -> &[Row] {
//         &self.rows
//     }

//     pub fn span(&self) -> Span {
//         self.span
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Row {
//     cells: Vec<Cell>,
//     span: Span,
// }

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(BigInt),
    Float(Float),
    Rational(BigRational),
    Str(InternedString),
    Bool(bool),
    Char(char),
}
