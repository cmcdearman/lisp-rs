use self::r#macro::{Macro, MacroCall};
use crate::read::sexpr::{Atom, AtomKind, Sexpr, SexprKind};

pub mod r#macro;

// fn extract_macros(sexpr: &Sexpr) -> Vec<Macro> {
//     todo!()
// }

// fn extract_macro_calls(sexpr: &Sexpr) -> Vec<MacroCall> {
//     todo!()
// }

pub fn expand(sexpr: &Sexpr) -> Sexpr {
    // let mut macros = vec![];
    // let mut calls = vec![];
    match sexpr.kind() {
        SexprKind::Atom(a) => match a.kind() {
            AtomKind::Sym(s) => match &**s {
                "macro" => todo!(),
                _ => todo!(),
            },
            _ => todo!(),
        },
        _ => todo!(),
    }
}
