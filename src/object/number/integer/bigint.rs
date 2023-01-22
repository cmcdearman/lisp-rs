use std::ops::Add;

use num_bigint::{BigInt, BigUint};

use crate::object::number::rational::Rational;

use super::fixnum::FixNum;

pub type BigNum = BigInt;

pub trait AbsExt {
    fn abs(&self) -> Self;
}

impl AbsExt for BigNum {
    fn abs(&self) -> Self {
        let (_sign, uint) = self.clone().into_parts();
        if uint == BigUint::default() {
            return Self::from_biguint(num_bigint::Sign::NoSign, uint);
        }
        Self::from_biguint(num_bigint::Sign::Plus, uint)
    }
}

impl Add<FixNum> for BigNum {
    type Output = Self;

    fn add(self, rhs: FixNum) -> Self::Output {
        self + BigNum::from(rhs)
    }
}

impl Add<Rational> for BigNum {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        Rational::from(self) + rhs
    }
}

impl From<FixNum> for BigNum {
    fn from(value: FixNum) -> Self {
        Self::from(value.0)
    }
}
