use self::r#macro::{Macro, MacroCall};
use crate::read::sexpr::{Atom, AtomKind, Root, Sexpr, SexprKind, SynList};
use lust_utils::{list::List, span::Span};

pub mod r#macro;

fn extract_macro(list: &List<Sexpr>, span: Span) -> Macro {
    let mut iter = list.iter();
    let (name, params) = match iter.next() {
        Some(s) => match s.kind() {
            SexprKind::SynList(param_list) => {
                let mut iter = param_list.list().iter();
                let name = match iter.next() {
                    Some(s) => match s.kind() {
                        SexprKind::Atom(a) => match a.kind() {
                            AtomKind::Sym(s) => s.clone(),
                            _ => panic!("macro name must be a symbol"),
                        },
                        _ => panic!("macro name must be a symbol"),
                    },
                    None => panic!("macro name must be a symbol"),
                };
                let mut params = vec![];
                while let Some(s) = iter.next() {
                    match s.kind() {
                        SexprKind::Atom(a) => match a.kind() {
                            AtomKind::Sym(s) => params.push(s.clone()),
                            _ => panic!("macro parameter must be a symbol"),
                        },
                        _ => panic!("macro parameter must be a symbol"),
                    }
                }
                (name, params)
            }
            _ => panic!("macro name must be a symbol"),
        },
        None => panic!("macro must have param list"),
    };
    let body = match iter.next() {
        Some(s) => s.clone(),
        None => panic!("macro body must be a sexpr"),
    };
    Macro::new(name, params, body, span)
}

pub fn collect_macros(root: &Root) -> Vec<Macro> {
    let mut macros = vec![];
    for sexpr in root.sexprs() {
        match sexpr.kind() {
            SexprKind::SynList(list) => match list.head() {
                Some(h) => match h.kind() {
                    SexprKind::Atom(a) => match a.kind() {
                        AtomKind::Sym(s) => {
                            if &**s == "macro" {
                                match list.tail() {
                                    Some(t) => macros.push(extract_macro(t, *list.span())),
                                    None => panic!("macro must have a body"),
                                }
                            }
                        }
                        _ => continue,
                    },
                    _ => continue,
                },
                None => continue,
            },
            _ => {}
        }
    }
    macros
}
