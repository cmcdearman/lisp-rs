use std::{cell::RefCell, rc::Rc, fmt::Display};

use super::cons::Cons;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    head: Option<Rc<RefCell<Cons>>>
}

impl Display for List {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(head) = &self.head {
            write!(formatter, "({})", head.as_ref().borrow())
        } else {
            write!(formatter, "NIL")
        }
    }
}