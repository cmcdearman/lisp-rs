use crate::ast::{object::Object, symbol::Symbol};
use std::{collections::HashMap, cell::RefCell, rc::Rc};

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

    pub fn push(&mut self, name: Symbol, value: Object) {
        self.entries.insert(name, value);
    }

    pub fn find(&self, name: Symbol) -> Result<Object, String> {
        match self.entries.get(&name) {
            Some(obj) => Ok(obj.clone()),
            None => Err(String::from("could not find name in env"))
        }
    }

    pub fn pop(&mut self, name: Symbol) {
        self.entries.remove(&name);
    }
}
