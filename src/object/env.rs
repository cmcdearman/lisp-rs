use crate::object::{Object, symbol::Symbol};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    entries: HashMap<Symbol, Object>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            parent: None,
            entries: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: Symbol, value: Object) {
        self.entries.insert(name, value);
    }

    pub fn find(&self, name: &Symbol) -> Option<Object> {
        if let Some(v) = self.entries.get(name) {
            Some(v.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().find(name)
        } else {
            None
        }
    }

    pub fn remove(&mut self, name: Symbol) {
        self.entries.remove(&name);
    }
}
