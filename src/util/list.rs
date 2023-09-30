use chumsky::container::Container;
use itertools::join;
use std::fmt::Display;

/// A singly-linked list with owned nodes.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum List<T> {
    Node(Box<(T, List<T>)>),
    // Node {
    //     car: Box<T>,
    //     cdr: Box<List<T>>,
    // },
    #[default]
    Nil,
}

impl<T> List<T> {
    pub fn new(car: T, cdr: List<T>) -> Self {
        Self::Node(Box::new((car, cdr)))
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct ConsPair<T> {
//     pub car: T,
//     pub cdr: ConsPair<T>,
// }

impl<T> Display for List<T>
where
    T: Display + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            List::Node(_) => {
                write!(f, "[")?;
                write!(f, "{}", join(self.clone(), ", "))?;
                write!(f, "]")
            }
            List::Nil => write!(f, "[]"),
        }
    }
}

impl<T: Clone> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            List::Node(n) => {
                let (data, next) = n.as_ref();
                let data = data.clone();
                *self = next.clone();
                Some(data)
            }
            List::Nil => None,
        }
    }
}

impl<T: Clone> ExactSizeIterator for List<T> {
    fn len(&self) -> usize {
        self.clone().count()
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter()
            .fold(Self::Nil, |list, data| Self::Node(Box::new((data, list))))
    }
}

impl<T: Clone> DoubleEndedIterator for List<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            List::Node(n) => {
                let (data, next) = n.as_ref();
                let data = data.clone();
                *self = next.clone();
                Some(data)
            }
            List::Nil => None,
        }
    }
}

impl<T: Clone + Default> Container<T> for List<T> {
    fn push(&mut self, item: T) {
        *self = Self::Node(Box::new((item, self.clone())));
    }

    fn with_capacity(n: usize) -> Self {
        let mut list = Self::Nil;
        for _ in 0..n {
            list = Self::Node(Box::new((Default::default(), list)));
        }
        list
    }
}
