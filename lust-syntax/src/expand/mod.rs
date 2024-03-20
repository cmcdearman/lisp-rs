use self::{
    r#macro::{Macro, MacroCall},
    store::MacroStore,
};
use crate::read::sexpr::{Atom, AtomKind, Root, Sexpr, SexprKind};
use lust_utils::{intern::InternedString, list::List, span::Span};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub mod env;
pub mod r#macro;
pub mod store;

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

fn collect_macros(mut store: MacroStore, root: &Root) {
    for sexpr in root.sexprs() {
        let kind = match sexpr.kind() {
            SexprKind::SynList(list) => list,
            _ => continue,
        };

        let head = match kind.head() {
            Some(h) => h,
            None => continue,
        };

        let atom_kind = match head.kind() {
            SexprKind::Atom(a) => a,
            _ => continue,
        };

        let sym = match atom_kind.kind() {
            AtomKind::Sym(s) => s,
            _ => continue,
        };

        if &**sym != "macro" {
            continue;
        }

        let tail = match kind.tail() {
            Some(t) => t,
            None => panic!("macro must have a body"),
        };

        let m = extract_macro(tail, kind.span());
        store.insert(m);
    }
}

pub fn expand_macros(store: MacroStore, root: &Root) -> Root {
    collect_macros(store.clone(), root);
    let mut sexprs = vec![];
    for sexpr in root.sexprs() {
        let kind = match sexpr.kind() {
            SexprKind::SynList(list) => list,
            _ => {
                sexprs.push(sexpr.clone());
                continue;
            }
        };

        let head = match kind.head() {
            Some(h) => h,
            None => {
                sexprs.push(sexpr.clone());
                continue;
            }
        };

        let atom_kind = match head.kind() {
            SexprKind::Atom(a) => a,
            _ => {
                sexprs.push(sexpr.clone());
                continue;
            }
        };

        let sym = match atom_kind.kind() {
            AtomKind::Sym(s) => s,
            _ => {
                sexprs.push(sexpr.clone());
                continue;
            }
        };

        if let Some(m) = store.get(sym) {
            let tail = match kind.tail() {
                Some(t) => t,
                None => panic!("macro must have a body"),
            };

            let mut args = vec![];
            for arg in tail.iter() {
                args.push(arg.clone());
            }
            let mut body = m.body().clone();
            for (i, param) in m.params().iter().enumerate() {
                body.replace_sym(param.clone(), args[i].clone());
            }
            sexprs.push(body);
        } else {
            sexprs.push(sexpr.clone());
        }
    }
    Root::new(sexprs, *root.span())
}
