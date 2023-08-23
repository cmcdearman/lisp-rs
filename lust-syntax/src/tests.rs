use super::{
    expr,
    reader::sexpr::{Atom, Lit, Sexpr},
};
use crate::span::Span;

#[test]
fn test_parse_int() {
    let sexpr = (Sexpr::Atom(Atom::Lit(Lit::Int(123))), Span::from(0..3));
    let expr = expr(&sexpr).expect("Failed to parse");
    insta::assert_debug_snapshot!(expr);
}
