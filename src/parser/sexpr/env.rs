use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::Sexpr;

#[derive(Debug, Clone)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    entries: HashMap<String, Sexpr>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            parent: None,
            entries: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Sexpr) {
        self.entries.insert(name, value);
    }

    pub fn find(&self, name: &String) -> Option<Sexpr> {
        if let Some(v) = self.entries.get(name) {
            Some(v.clone())
        } else if let Some(parent) = &self.parent {
            parent.as_ref().borrow().find(name)
        } else {
            None
        }
    }

    pub fn remove(&mut self, name: String) {
        self.entries.remove(&name);
    }
}

impl PartialEq for Env {
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent && self.entries == other.entries
    }
}
