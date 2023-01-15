use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Rem, Sub},
};

use crate::ast::number::Number;

use super::{Integer, bignum::BigNum};

#[derive(Debug, Clone)]
pub struct FixNum(pub i64);

impl Display for FixNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FixNum(n) => write!(f, "{}", n),
        }
    }
}

impl Add for FixNum {
    type Output = Integer;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum(i), FixNum(z)) => match i.checked_add(z) {
                Some(n) => Integer::FixNum(Self(n)),
                None => Integer::BigNum(BigNum::from(i) + BigNum::from(z)),
            },
        }
    }
}

impl Add<i64> for FixNum {
    type Output = Integer;

    fn add(self, rhs: i64) -> Self::Output {
        match (self, rhs) {
            (FixNum(i), z) => match i.checked_add(z) {
                Some(n) => Integer::FixNum(Self(n)),
                None => Integer::BigNum(BigNum::from(i) + BigNum::from(z)),
            },
        }
    }
}

impl Add<f64> for FixNum {
    type Output = f64;

    fn add(self, rhs: f64) -> Self::Output {
        match (self, rhs) {
            (FixNum(i), z) => i as f64 + z,
        }
    }
}

impl Sub for FixNum {
    type Output = Integer;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum(i), FixNum(z)) => match i.checked_sub(z) {
                Some(n) => Integer::FixNum(Self(n)),
                None => Integer::BigNum(BigNum::from(i) - BigNum::from(z)),
            },
        }
    }
}

impl Mul for FixNum {
    type Output = Integer;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum(i), FixNum(z)) => match i.checked_mul(z) {
                Some(n) => Integer::FixNum(Self(n)),
                None => Integer::BigNum(BigNum::from(i) * BigNum::from(z)),
            },
        }
    }
}

impl Div for FixNum {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum(i), FixNum(z)) => match i.checked_div(z) {
                Some(n) => Number::Integer(Integer::FixNum(Self(n))),
                None => Number::Integer(Integer::BigNum(BigNum::from(i) / BigNum::from(z))),
            },
        }
    }
}

impl Rem for FixNum {
    type Output = Integer;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FixNum(i), FixNum(z)) => match i.checked_rem(z) {
                Some(n) => Integer::FixNum(Self(n)),
                None => Integer::BigNum(BigNum::from(i) % BigNum::from(z)),
            },
        }
    }
}

impl From<i64> for FixNum {
    fn from(value: i64) -> Self {
        FixNum(value)
    }
}

impl From<FixNum> for i64 {
    fn from(value: FixNum) -> Self {
        value.0
    }
}

impl From<FixNum> for f64 {
    fn from(value: FixNum) -> Self {
        value.0 as f64
    }
}
