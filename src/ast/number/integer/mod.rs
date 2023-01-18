use std::{fmt::Display, ops::{Add, Sub, Mul, Rem, Div}, iter::{Sum, Product}};

use self::{fixnum::FixNum, bignum::BigNum};

use super::rational::Rational;

pub mod fixnum;
pub mod bignum;

#[derive(Debug, Clone)]
pub enum Integer {
    FixNum(FixNum),
    BigNum(BigNum),
}

impl Integer {
    pub fn abs(&self) -> Self {
        match self {
            Integer::FixNum(n) => Integer::FixNum(n.abs()),
            Integer::BigNum(n) => Integer::BigNum(n.abs()),
        }
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer::FixNum(n) => write!(f, "{}", n),
            Integer::BigNum(n) => write!(f, "{}", n),
        }
    }
}

impl Add for Integer {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::FixNum(n1), Integer::FixNum(n2)) => n1 + n2,
            (Integer::FixNum(n1), Integer::BigNum(n2)) => Self::BigNum(BigNum::from(n1) + n2),
            (Integer::BigNum(n1), Integer::FixNum(n2)) => Self::BigNum(n1 + BigNum::from(n2)),
            (Integer::BigNum(n1), Integer::BigNum(n2)) => Self::BigNum(n1 + n2),
        }
    }
}

// impl Add<f64> for Integer {
//     type Output = f64;

//     fn add(self, rhs: f64) -> Self::Output {
//         match (self, rhs) {
//             (Integer::FixNum(n1), n2) => n1 + n2,
//             (Integer::BigNum(n1), n2) => Self::BigNum(n1 + BigNum::from(n2)),
//         }
//     }
// }

impl Sub for Integer {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::FixNum(n1), Integer::FixNum(n2)) => n1 - n2,
            (Integer::FixNum(n1), Integer::BigNum(n2)) => Self::BigNum(BigNum::from(n1) - n2),
            (Integer::BigNum(n1), Integer::FixNum(n2)) => Self::BigNum(n1 - BigNum::from(n2)),
            (Integer::BigNum(n1), Integer::BigNum(n2)) => Self::BigNum(n1 - n2),
        }
    }
}

impl Mul for Integer {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::FixNum(n1), Integer::FixNum(n2)) => n1 * n2,
            (Integer::FixNum(n1), Integer::BigNum(n2)) => Self::BigNum(BigNum::from(n1) * n2),
            (Integer::BigNum(n1), Integer::FixNum(n2)) => Self::BigNum(n1 * BigNum::from(n2)),
            (Integer::BigNum(n1), Integer::BigNum(n2)) => Self::BigNum(n1 * n2),
        }
    }
}

impl Div for Integer {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::FixNum(n1), Integer::FixNum(n2)) => {
                let r = Rational::from(n1) / Rational::from(n2);
            },
            (Integer::FixNum(n1), Integer::BigNum(n2)) => Self::BigNum(BigNum::from(n1) / n2),
            (Integer::BigNum(n1), Integer::FixNum(n2)) => Self::BigNum(n1 / BigNum::from(n2)),
            (Integer::BigNum(n1), Integer::BigNum(n2)) => Self::BigNum(n1 / n2),
        }
    }
}

impl Rem for Integer {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::FixNum(n1), Integer::FixNum(n2)) => n1 % n2,
            (Integer::FixNum(n1), Integer::BigNum(n2)) => Self::BigNum(BigNum::from(n1) % n2),
            (Integer::BigNum(n1), Integer::FixNum(n2)) => Self::BigNum(n1 % BigNum::from(n2)),
            (Integer::BigNum(n1), Integer::BigNum(n2)) => Self::BigNum(n1 % n2),
        }
    }
}

impl Sum for Integer {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Integer::FixNum(FixNum(0)), |a, b| a + b)
    }
}

impl Product for Integer {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Integer::FixNum(FixNum(1)), |a, b| a * b)
    }
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Integer::FixNum(FixNum(value))
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // (Self::FixNum(l0), Self::FixNum(r0)) => l0 == r0,
            // (Self::BigNum(l0), Self::BigNum(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Integer{}