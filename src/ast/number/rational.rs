use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
};

use super::integer::{fixnum::FixNum, Integer};

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
        if self.numerator == self.denominator {
            return Self { numerator: Integer::FixNum(FixNum::from(1)), denominator: Integer::FixNum(FixNum::from(1)) };
        }
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
        Self {
            numerator: value,
            denominator: Integer::FixNum(FixNum::from(1)),
        }
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        }
        .normalize()
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator - rhs.numerator,
            denominator: self.denominator - rhs.denominator,
        }
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl Rem for Rational {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator % rhs.numerator,
            denominator: self.denominator % rhs.denominator,
        }
    }
}
