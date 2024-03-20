use super::r#macro::Macro;
use lust_utils::intern::InternedString;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct MacroStore {
    macros: Rc<RefCell<HashMap<InternedString, Macro>>>,
}

impl MacroStore {
    pub fn new() -> Self {
        Self {
            macros: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn insert(&mut self, macro_: Macro) {
        self.macros
            .borrow_mut()
            .insert(macro_.name().clone(), macro_);
    }

    pub fn get(&self, name: &InternedString) -> Option<Macro> {
        self.macros.borrow().get(name).cloned()
    }
}
