use std::{
    fmt::{Debug, Display},
    iter::{Product, Sum},
    ops::{Add, Div, Mul, Rem, Sub},
};

#[derive(Debug, Clone)]
pub enum Number {
    FixNum(FixNum),
    // BigNum(BigNum),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::FixNum(n) => write!(f, "{}", n),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::FixNum(n1), Number::FixNum(n2)) => Self::FixNum(n1 + n2),
        }
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::FixNum(n1), Number::FixNum(n2)) => Self::FixNum(n1 - n2),
        }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::FixNum(n1), Number::FixNum(n2)) => Self::FixNum(n1 * n2),
        }
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::FixNum(n1), Number::FixNum(n2)) => Self::FixNum(n1 / n2),
        }
    }
}

impl Rem for Number {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::FixNum(n1), Number::FixNum(n2)) => Self::FixNum(n1 % n2),
        }
    }
}

impl Sum for Number {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Number::FixNum(FixNum::Integer(0)), |a, b| a + b)
    }
}

impl Product for Number {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Number::FixNum(FixNum::Integer(0)), |a, b| a * b)
    }
}

#[derive(Debug, Clone)]
pub enum FixNum {
    Integer(i64),
    Float(f64),
}

impl Display for FixNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FixNum::Integer(n) => write!(f, "{}", n),
            FixNum::Float(n) => write!(f, "{}", n),
        }
    }
}

impl Add for FixNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum::Integer(i), FixNum::Integer(z)) => Self::Integer(i + z),
            (FixNum::Integer(i), FixNum::Float(f)) => Self::Float(i as f64 + f),
            (FixNum::Float(f), FixNum::Integer(i)) => Self::Float(f + i as f64),
            (FixNum::Float(f1), FixNum::Float(f2)) => Self::Float(f1 + f2),
        }
    }
}

impl Sub for FixNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum::Integer(i), FixNum::Integer(z)) => Self::Integer(i - z),
            (FixNum::Integer(i), FixNum::Float(f)) => Self::Float(i as f64 - f),
            (FixNum::Float(f), FixNum::Integer(i)) => Self::Float(f - i as f64),
            (FixNum::Float(f1), FixNum::Float(f2)) => Self::Float(f1 - f2),
        }
    }
}

impl Mul for FixNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum::Integer(i), FixNum::Integer(z)) => Self::Integer(i * z),
            (FixNum::Integer(i), FixNum::Float(f)) => Self::Float(i as f64 * f),
            (FixNum::Float(f), FixNum::Integer(i)) => Self::Float(f * i as f64),
            (FixNum::Float(f1), FixNum::Float(f2)) => Self::Float(f1 * f2),
        }
    }
}

impl Div for FixNum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum::Integer(i), FixNum::Integer(z)) => Self::Integer(i / z),
            (FixNum::Integer(i), FixNum::Float(f)) => Self::Float(i as f64 / f),
            (FixNum::Float(f), FixNum::Integer(i)) => Self::Float(f / i as f64),
            (FixNum::Float(f1), FixNum::Float(f2)) => Self::Float(f1 / f2),
        }
    }
}

impl Rem for FixNum {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum::Integer(i), FixNum::Integer(z)) => Self::Integer(i % z),
            (FixNum::Integer(i), FixNum::Float(f)) => Self::Float(i as f64 % f),
            (FixNum::Float(f), FixNum::Integer(i)) => Self::Float(f % i as f64),
            (FixNum::Float(f1), FixNum::Float(f2)) => Self::Float(f1 % f2),
        }
    }
}

impl Sum for FixNum {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(FixNum::Integer(0), |a, b| a + b)
    }
}

impl Product for FixNum {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(FixNum::Integer(0), |a, b| a * b)
    }
}

#[derive(Debug, Clone)]
pub enum BigNum {
    Integer(Vec<u8>),
    Float(Vec<u8>),
}
