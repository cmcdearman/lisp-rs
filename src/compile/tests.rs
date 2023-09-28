use crate::{
    parser::ast::{Expr, Lit},
    span::{Span, Spanned},
};

#[test]
fn test_compile_int() {
    let expr = (Expr::Lit(Lit::Int(123)), Span::from(0..0));
    let mut c = crate::compiler::Compiler::new();
    let chunk = c.compile(&expr).expect("Failed to compile");
    insta::assert_debug_snapshot!(chunk);
}
