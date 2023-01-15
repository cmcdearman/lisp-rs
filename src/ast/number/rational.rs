use std::fmt::Display;

use super::integer::Integer;

#[derive(Debug, Clone)]
pub struct Rational {
    numerator: Integer,
    denominator: Integer,
}

impl Rational {
    pub fn new(numerator: Integer, denominator: Integer) -> Self {
        Self {
            numerator,
            denominator,
        }
        .normalize()
    }

    fn normalize(&self) -> Self {
        todo!()
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}
