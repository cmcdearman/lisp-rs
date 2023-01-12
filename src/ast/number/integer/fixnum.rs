use std::{
    fmt::{Debug, Display},
    iter::{Product, Sum},
    ops::{Add, Div, Mul, Rem, Sub},
};


#[derive(Debug, Clone)]
pub struct FixNum(i64);

impl Display for FixNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FixNum(n) => write!(f, "{}", n),
        }
    }
}

impl Add for FixNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum(i), FixNum(z)) => Self(i + z),
        }

    }
}

impl Sub for FixNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum(i), FixNum(z)) => Self(i - z),
        }
    }
}

impl Mul for FixNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum(i), FixNum(z)) => Self(i * z),
        }
    }
}

impl Div for FixNum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum(i), FixNum(z)) => Self(i / z),
        }
    }
}

impl Rem for FixNum {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum(i), FixNum(z)) => Self(i % z),
        }
    }
}

impl Sum for FixNum {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(FixNum(0), |a, b| a + b)
    }
}

impl Product for FixNum {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(FixNum(1), |a, b| a * b)
    }
}

#[derive(Debug, Clone)]
pub enum BigNum {
    Integer(Vec<u8>),
    Float(Vec<u8>),
}
