use std::{cell::RefCell, fmt::{Display, Debug}, rc::Rc};

use super::{cons::Cons, object::Object};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    head: Option<Rc<RefCell<Cons>>>,
}

impl List {
    pub const NIL: List = List { head: None };

    pub fn car(&self) -> Result<Object, String> {
        self.head
            .as_ref()
            .map(|rc| rc.borrow().car.clone())
            .ok_or_else(|| String::from("Attempted to apply car on nil"))
    }

    #[must_use]
    pub fn cdr(&self) -> List {
        List {
            head: self
                .head
                .as_ref()
                .and_then(|rc| rc.borrow().cdr.as_ref().cloned()),
        }
    }

    #[must_use]
    pub fn cons(&self, val: Object) -> List {
        List {
            head: Some(Rc::new(RefCell::new(Cons {
                car: val,
                cdr: self.head.clone(),
            }))),
        }
    }
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