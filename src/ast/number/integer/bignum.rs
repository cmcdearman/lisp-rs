use num_bigint::BigInt;

use super::fixnum::FixNum;

pub type BigNum = BigInt;

pub trait AbsExt {
    fn abs(&mut self);
}

impl AbsExt for BigNum {
    fn abs(&mut self) {
        let this = std::mem::take(self);
        let (_sign, uint) = this.into_parts();
        *self = Self::from_biguint(num_bigint::Sign::Plus, uint);
    }
}

impl From<FixNum> for BigNum {
    fn from(value: FixNum) -> Self {
        Self::from(value.0)
    }
}
