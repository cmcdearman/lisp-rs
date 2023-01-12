pub mod integer;

use std::{fmt::Display, ops::{Add, Sub, Mul, Div, Rem}, iter::{Sum, Product}};

use self::integer::Integer;



#[derive(Debug, Clone)]
pub enum Number {
    Integer(Integer),
    // Float(f64),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Integer(n) => write!(f, "{}", n),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(n1), Number::Integer(n2)) => Self::Integer(n1 + n2),
        }
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(n1), Number::Integer(n2)) => Self::Integer(n1 - n2),
        }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(n1), Number::Integer(n2)) => Self::Integer(n1 * n2),
        }
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(n1), Number::Integer(n2)) => Self::Integer(n1 / n2),
        }
    }
}

impl Rem for Number {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(n1), Number::Integer(n2)) => Self::Integer(n1 % n2),
        }
    }
}

impl Sum for Number {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Number::Integer(Integer::Integer(0)), |a, b| a + b)
    }
}

impl Product for Number {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Number::Integer(Integer::Integer(1)), |a, b| a * b)
    }
}