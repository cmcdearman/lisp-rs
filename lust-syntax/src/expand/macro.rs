use crate::read::sexpr::Sexpr;
use itertools::Itertools;
use lust_utils::{intern::InternedString, span::Span};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Macro {
    name: InternedString,
    params: Vec<InternedString>,
    body: Sexpr,
    span: Span,
}

impl Macro {
    pub fn new(name: InternedString, params: Vec<InternedString>, body: Sexpr, span: Span) -> Self {
        Self {
            name,
            params,
            body,
            span,
        }
    }

    pub fn name(&self) -> &InternedString {
        &self.name
    }

    pub fn params(&self) -> &[InternedString] {
        &self.params
    }

    pub fn body(&self) -> &Sexpr {
        &self.body
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl Display for Macro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(macro ({} {}) {})",
            self.name,
            self.params.join(" "),
            self.body
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroCall {
    name: InternedString,
    args: Vec<Sexpr>,
    span: Span,
}

impl MacroCall {
    pub fn new(name: InternedString, args: Vec<Sexpr>, span: Span) -> Self {
        Self { name, args, span }
    }

    pub fn name(&self) -> &InternedString {
        &self.name
    }

    pub fn args(&self) -> &[Sexpr] {
        &self.args
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl Display for MacroCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.name, self.args.iter().join(" "))
    }
}
