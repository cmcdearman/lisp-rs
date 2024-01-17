use self::r#macro::{Macro, MacroCall};
use crate::read::sexpr::{Atom, AtomKind, Root, Sexpr, SexprKind, SynList};
use lust_utils::{intern::InternedString, list::List, span::Span};
use std::collections::HashMap;

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

pub fn collect_macros(root: &Root) -> HashMap<InternedString, Macro> {
    let mut macros = HashMap::new();
    for sexpr in root.sexprs() {
        match sexpr.kind() {
            SexprKind::SynList(list) => match list.head() {
                Some(h) => match h.kind() {
                    SexprKind::Atom(a) => match a.kind() {
                        AtomKind::Sym(s) => {
                            if &**s == "macro" {
                                match list.tail() {
                                    Some(t) => {
                                        let m = extract_macro(t, *list.span());
                                        macros.insert(m.name().clone(), m);
                                    }
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

pub fn collect_calls(root: &Root, macros: HashMap<InternedString, Macro>) -> Vec<MacroCall> {
    let mut calls = vec![];
    for sexpr in root.sexprs() {
        match sexpr.kind() {
            SexprKind::SynList(list) => match list.head() {
                Some(h) => match h.kind() {
                    SexprKind::Atom(a) => match a.kind() {
                        AtomKind::Sym(s) => {
                            if let Some(m) = macros.get(s) {
                                match list.tail() {
                                    Some(t) => {
                                        let mut args = vec![];
                                        for arg in t.iter() {
                                            args.push(arg.clone());
                                        }
                                        calls.push(MacroCall::new(
                                            m.name().clone(),
                                            args,
                                            *list.span(),
                                        ));
                                    }
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
    calls
}

pub fn expand_macros(root: &Root) -> Root {
    let macros = collect_macros(root);
    let calls = collect_calls(root, macros.clone());
    for call in calls {
        if let Some(m) = macros.get(call.name()) {
            
        } else {
            panic!("macro {} not found", call.name());
        }
    }
    todo!()
}
