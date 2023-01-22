use std::{cell::RefCell, fmt::Display, rc::Rc};

use super::Object;

#[derive(Debug, PartialEq, Eq)]
pub struct Cons {
    pub car: Object,
    pub cdr: Option<Rc<RefCell<Cons>>>,
}

impl Display for Cons {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.cdr.as_ref() {
            Some(cdr) => write!(formatter, "{} {}", self.car, cdr.borrow()),
            None => write!(formatter, "{}", self.car),
        }
    }
}

#[derive(Clone)]
pub struct ConsIterator(pub Option<Rc<RefCell<Cons>>>);

impl Iterator for ConsIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.clone().map(|cons| {
            let val = cons.borrow().car.clone();

            self.0 = cons.borrow().cdr.clone();

            val
        })
    }
}

impl ExactSizeIterator for ConsIterator {
    fn len(&self) -> usize {
        let mut length: usize = 0;

        self.clone().for_each(|_| length += 1);

        length
    }
}