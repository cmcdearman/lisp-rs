use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

use crate::parser;

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

    pub fn create_child(parent: Rc<RefCell<Self>>) -> Self {
        Self {
            parent: Some(parent),
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

    // pub fn dump_entries(&self) -> HashMap<String, Sexpr> {
    //     self.entries.clone()
    // }

    // pub fn dump_all_entries(&self) -> HashMap<String, HashMap<String, Sexpr>> {
    //     let mut tables = HashMap::new();
    //     tables.insert(self.tag.to_string(), self.dump_entries());
    //     if let Some(parent) = &self.parent {
    //         tables = tables
    //             .into_iter()
    //             .chain(parent.as_ref().borrow().dump_all_entries())
    //             .collect();
    //     }
    //     tables
    // }

    pub fn get_parent(&self) -> Option<Rc<RefCell<Env>>> {
        self.parent.clone()
    }
}

impl PartialEq for Env {
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent && self.entries == other.entries
    }
}
