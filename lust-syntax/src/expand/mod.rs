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
                            if &**s == "macro" {
                                continue;
                            }
                            calls.push(MacroCall::new(
                                s.clone(),
                                list.tail()
                                    .unwrap_or(&List::new())
                                    .iter()
                                    .cloned()
                                    .collect(),
                                *list.span(),
                            ));
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
    todo!()
    // let mut root = root.clone();
    // let macros = collect_macros(&root);
    // let calls = collect_calls(&root);
    // for call in calls {
    //     let mut expanded = false;
    //     for m in &macros {
    //         if &*call.name() == m.name() {
    //             let mut body = m.body().clone();
    //             for (i, param) in m.params().iter().enumerate() {
    //                 body = body.replace(param, call.args()[i].clone());
    //             }
    //             root.replace(&call, body);
    //             expanded = true;
    //             break;
    //         }
    //     }
    //     if !expanded {
    //         panic!("macro {} not found", call.name());
    //     }
    // }
    // root
}
