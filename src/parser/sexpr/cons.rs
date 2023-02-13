use std::{cell::RefCell, fmt::Display, rc::Rc};

use super::Sexpr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cons {
    pub car: Sexpr,
    pub cdr: Option<Box<Cons>>,
}

impl Display for Cons {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.cdr.as_ref() {
            Some(cdr) => write!(formatter, "{} {}", self.car, cdr),
            None => write!(formatter, "{}", self.car),
        }
    }
}

#[derive(Clone)]
pub struct ConsIterator(pub Option<Box<Cons>>);

impl Iterator for ConsIterator {
    type Item = Sexpr;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.clone().map(|cons| {
            let val = cons.car.clone();

            self.0 = cons.cdr.clone();

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