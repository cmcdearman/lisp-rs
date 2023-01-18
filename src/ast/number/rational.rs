use std::{fmt::Display, ops::{Add, Sub, Mul, Div, Rem}};

use super::integer::{Integer, fixnum::FixNum};

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

impl From<Integer> for Rational {
    fn from(value: Integer) -> Self {
        Self { numerator: value, denominator: Integer::FixNum(FixNum::from(1)) }
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Self { numerator: }
        todo!()
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // Self { numerator: }
        todo!()
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // Self { numerator: }
        todo!()
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // Self { numerator: }
        todo!()
    }
}

impl Rem for Rational {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        // Self { numerator: }
        todo!()
    }
}