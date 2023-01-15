use std::fmt::{Display, Write};

use super::integer::Integer;

#[derive(Debug, Clone)]
pub struct Rational(Sign, Integer, Integer);

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}/{}", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Sign {
    Plus,
    Minus,
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sign::Plus => f.write_char('+'),
            Sign::Minus => f.write_char('-'),
        }
    }
}

