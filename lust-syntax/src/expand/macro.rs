use crate::read::sexpr::Sexpr;
use lust_utils::{intern::InternedString, span::Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Macro {
    name: InternedString,
    args: Vec<InternedString>,
    body: Sexpr,
    span: Span,
}
