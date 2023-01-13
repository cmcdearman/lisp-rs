use num_bigint::BigInt;

use super::fixnum::FixNum;

pub type BigNum = BigInt; 

impl From<FixNum> for BigNum {
    fn from(value: FixNum) -> Self {
        BigNum::from(value.0)
    }
}
