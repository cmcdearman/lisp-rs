use std::{fmt::Display, ops::{Add, Sub, Mul, Rem, Div}, iter::{Sum, Product}};

use self::{fixnum::FixNum, bigint::{BigNum, AbsExt}};

use super::{rational::Rational, Number};

pub mod fixnum;
pub mod bigint;

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

impl Add<Rational> for Integer {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        match (self, rhs) {
            (Integer::FixNum(l), r) => l + r,
            (Integer::BigNum(l), r) => l + r,
        }
    }
}

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
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::FixNum(n1), Integer::FixNum(n2)) => {
                let r = Rational::from(n1) / Rational::from(n2);
                if r.is_integer() {
                    return Number::Integer(Integer::try_from(r).expect("somehow is_integer failed to catch this"));
                }
                Number::Rational(r)
            },
            (Integer::FixNum(n1), Integer::BigNum(n2)) => Number::Integer(Self::BigNum(BigNum::from(n1) / n2)),
            (Integer::BigNum(n1), Integer::FixNum(n2)) => Number::Integer(Self::BigNum(n1 / BigNum::from(n2))),
            (Integer::BigNum(n1), Integer::BigNum(n2)) => Number::Integer(Self::BigNum(n1 / n2)),
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
            (Self::FixNum(l0), Self::FixNum(r0)) => l0 == r0,
            (Self::BigNum(l0), Self::BigNum(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Integer{}

impl TryFrom<Rational> for Integer {
    type Error = String;

    fn try_from(value: Rational) -> Result<Self, Self::Error> {
        if !value.is_integer() {
            return Err("can't convert non-integer rational to fixnum".to_string())
        }
        Ok(value.numerator())
    }
}