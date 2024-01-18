use crate::read::sexpr::Sexpr;
use lust_utils::intern::InternedString;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct Env {
    bindings: HashMap<InternedString, Sexpr>,
    parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Env) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(Rc::new(RefCell::new(parent))),
        }
    }

    pub fn get(&self, name: &InternedString) -> Option<Sexpr> {
        self.bindings.get(name).cloned().or(self
            .parent
            .as_ref()
            .and_then(|parent| parent.borrow().get(name).clone()))
    }

    pub fn insert(&mut self, name: InternedString, value: Sexpr) {
        self.bindings.insert(name, value);
    }
}
