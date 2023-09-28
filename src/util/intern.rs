use std::{
    fmt::{Debug, Display},
    ops::{Index, Range},
};

use lasso::{Spur, ThreadedRodeo};
use once_cell::sync::Lazy;

use crate::span::Span;

pub static mut INTERNER: Lazy<ThreadedRodeo> = Lazy::new(|| ThreadedRodeo::default());

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InternedString {
    pub key: Spur,
}

impl InternedString {
    pub fn len(&self) -> usize {
        self.to_string().len()
    }
}

impl Debug for InternedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternedString({})", unsafe {
            INTERNER.resolve(&self.key)
        })
    }
}

impl Display for InternedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { INTERNER.resolve(&self.key) })
    }
}

impl From<Spur> for InternedString {
    fn from(key: Spur) -> Self {
        Self { key }
    }
}

impl From<&str> for InternedString {
    fn from(name: &str) -> Self {
        Self {
            key: unsafe { INTERNER.get_or_intern(name) },
        }
    }
}

impl Index<Range<usize>> for InternedString {
    type Output = str;

    fn index(&self, index: Range<usize>) -> &Self::Output {
        let s = unsafe { INTERNER.resolve(&self.key) };
        &s[index]
    }
}

impl Index<Span> for InternedString {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        let s = unsafe { INTERNER.resolve(&self.key) };
        &s[index]
    }
}
