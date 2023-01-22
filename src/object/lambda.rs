use super::{env::Env, Object, symbol::Symbol};
use std::{cell::RefCell, rc::Rc, fmt::Display};

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