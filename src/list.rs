use std::fmt::Display;

/// A singly-linked list with owned nodes.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub const NIL: List<T> = List { head: None };

    pub fn new(head: Option<Box<Node<T>>>) -> Self {
        Self { head }
    }
}

impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
            None => None,
        }
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(v: Vec<T>) -> Self {
        v.into_iter().rev().fold(Self::NIL, |list, data| {
            Self::new(Some(Box::new(Node {
                data,
                next: list.head,
            })))
        })
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Display for Node<T>
where
    T: Display + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut node = self;
        write!(f, "[")?;
        while let Some(next) = &node.next {
            write!(f, "{}", node.data)?;
            node = next;
            if node.next.is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}
