use num_bigint::BigInt;

use super::{fixnum::FixNum};

pub type BigNum = BigInt; 

// impl BigNum {
//     pub fn abs(&self) -> Self {
//         Self(self.0.abs())
//     }
// }

impl From<FixNum> for BigNum {
    fn from(value: FixNum) -> Self {
        BigNum::from(value.0)
    }
}

