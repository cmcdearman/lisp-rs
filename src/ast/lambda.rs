use std::{rc::Rc, cell::RefCell};
use super::{symbol::Symbol, object::Object, env::Env};

#[derive(Debug, Clone)]
pub struct Lambda {
    pub env: Rc<RefCell<Env>>,
    pub args: Vec<Symbol>,
    pub body: Rc<Object>,
}