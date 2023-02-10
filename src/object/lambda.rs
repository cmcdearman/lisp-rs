use super::{env::Env, symbol::Symbol, Object};
use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug, Clone)]
pub struct Lambda {
    pub env: Rc<RefCell<Env>>,
    pub args: Vec<Symbol>,
    pub body: Rc<Object>,
}

impl Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl PartialEq for Lambda {
    fn eq(&self, other: &Self) -> bool {
        self.env.borrow().clone() == other.env.borrow().clone()
            && self.args == other.args
            && self.body == other.body
    }
}
