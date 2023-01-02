use super::{env::Env, object::Object, symbol::Symbol};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Lambda {
    pub env: Rc<RefCell<Env>>,
    pub args: Vec<Symbol>,
    pub body: Rc<Object>,
}
