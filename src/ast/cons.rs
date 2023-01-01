use std::{rc::Rc, cell::RefCell, fmt::Display};

use super::object::Object;

#[derive(Debug, PartialEq, Eq)]
pub struct Cons {
    car: Object,
    cdr: Option<Rc<RefCell<Cons>>>
}

impl Display for Cons {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.cdr.as_ref() {
            Some(cdr) => write!(formatter, "{} {}", self.car, cdr.borrow()),
            None => write!(formatter, "{}", self.car),
        }
    }
}
