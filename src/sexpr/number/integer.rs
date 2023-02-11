use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Rem, Sub}, str::FromStr,
};

use num_bigint::{BigInt, BigUint};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseIntegerError(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Integer {
    Int8(Int8),
    UInt8(UInt8),
    Int16(Int16),
    UInt16(UInt16),
    Int32(Int32),
    UInt32(UInt32),
    Int64(Int64),
    UInt64(UInt64),
    BigInt(BigInt),
    BigUint(BigUint),
    Inf,
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Integer {
    type Err = ParseIntegerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       todo!() 
    }
}

impl Add for Integer {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::Int8(l), Integer::Int8(r)) => {
                if let Some(sum) = l.0.checked_add(r.0) {
                    return Integer::Int8(Int8(sum));
                } else {
                    return Integer::Int16(Int16(l.0 as i16 + r.0 as i16));
                }
            }
            (Integer::Int8(l), Integer::UInt8(r)) => {
                todo!()
            }
            (Integer::Int8(_), Integer::Int16(_)) => todo!(),
            (Integer::Int8(_), Integer::UInt16(_)) => todo!(),
            (Integer::Int8(_), Integer::Int32(_)) => todo!(),
            (Integer::Int8(_), Integer::UInt32(_)) => todo!(),
            (Integer::Int8(_), Integer::Int64(_)) => todo!(),
            (Integer::Int8(_), Integer::UInt64(_)) => todo!(),
            (Integer::Int8(_), Integer::BigInt(_)) => todo!(),
            (Integer::Int8(_), Integer::BigUint(_)) => todo!(),
            (Integer::Int8(_), Integer::Inf) => todo!(),
            (Integer::UInt8(_), Integer::Int8(_)) => todo!(),
            (Integer::UInt8(_), Integer::UInt8(_)) => todo!(),
            (Integer::UInt8(_), Integer::Int16(_)) => todo!(),
            (Integer::UInt8(_), Integer::UInt16(_)) => todo!(),
            (Integer::UInt8(_), Integer::Int32(_)) => todo!(),
            (Integer::UInt8(_), Integer::UInt32(_)) => todo!(),
            (Integer::UInt8(_), Integer::Int64(_)) => todo!(),
            (Integer::UInt8(_), Integer::UInt64(_)) => todo!(),
            (Integer::UInt8(_), Integer::BigInt(_)) => todo!(),
            (Integer::UInt8(_), Integer::BigUint(_)) => todo!(),
            (Integer::UInt8(_), Integer::Inf) => todo!(),
            (Integer::Int16(_), Integer::Int8(_)) => todo!(),
            (Integer::Int16(_), Integer::UInt8(_)) => todo!(),
            (Integer::Int16(_), Integer::Int16(_)) => todo!(),
            (Integer::Int16(_), Integer::UInt16(_)) => todo!(),
            (Integer::Int16(_), Integer::Int32(_)) => todo!(),
            (Integer::Int16(_), Integer::UInt32(_)) => todo!(),
            (Integer::Int16(_), Integer::Int64(_)) => todo!(),
            (Integer::Int16(_), Integer::UInt64(_)) => todo!(),
            (Integer::Int16(_), Integer::BigInt(_)) => todo!(),
            (Integer::Int16(_), Integer::BigUint(_)) => todo!(),
            (Integer::Int16(_), Integer::Inf) => todo!(),
            (Integer::UInt16(_), Integer::Int8(_)) => todo!(),
            (Integer::UInt16(_), Integer::UInt8(_)) => todo!(),
            (Integer::UInt16(_), Integer::Int16(_)) => todo!(),
            (Integer::UInt16(_), Integer::UInt16(_)) => todo!(),
            (Integer::UInt16(_), Integer::Int32(_)) => todo!(),
            (Integer::UInt16(_), Integer::UInt32(_)) => todo!(),
            (Integer::UInt16(_), Integer::Int64(_)) => todo!(),
            (Integer::UInt16(_), Integer::UInt64(_)) => todo!(),
            (Integer::UInt16(_), Integer::BigInt(_)) => todo!(),
            (Integer::UInt16(_), Integer::BigUint(_)) => todo!(),
            (Integer::UInt16(_), Integer::Inf) => todo!(),
            (Integer::Int32(_), Integer::Int8(_)) => todo!(),
            (Integer::Int32(_), Integer::UInt8(_)) => todo!(),
            (Integer::Int32(_), Integer::Int16(_)) => todo!(),
            (Integer::Int32(_), Integer::UInt16(_)) => todo!(),
            (Integer::Int32(_), Integer::Int32(_)) => todo!(),
            (Integer::Int32(_), Integer::UInt32(_)) => todo!(),
            (Integer::Int32(_), Integer::Int64(_)) => todo!(),
            (Integer::Int32(_), Integer::UInt64(_)) => todo!(),
            (Integer::Int32(_), Integer::BigInt(_)) => todo!(),
            (Integer::Int32(_), Integer::BigUint(_)) => todo!(),
            (Integer::Int32(_), Integer::Inf) => todo!(),
            (Integer::UInt32(_), Integer::Int8(_)) => todo!(),
            (Integer::UInt32(_), Integer::UInt8(_)) => todo!(),
            (Integer::UInt32(_), Integer::Int16(_)) => todo!(),
            (Integer::UInt32(_), Integer::UInt16(_)) => todo!(),
            (Integer::UInt32(_), Integer::Int32(_)) => todo!(),
            (Integer::UInt32(_), Integer::UInt32(_)) => todo!(),
            (Integer::UInt32(_), Integer::Int64(_)) => todo!(),
            (Integer::UInt32(_), Integer::UInt64(_)) => todo!(),
            (Integer::UInt32(_), Integer::BigInt(_)) => todo!(),
            (Integer::UInt32(_), Integer::BigUint(_)) => todo!(),
            (Integer::UInt32(_), Integer::Inf) => todo!(),
            (Integer::Int64(_), Integer::Int8(_)) => todo!(),
            (Integer::Int64(_), Integer::UInt8(_)) => todo!(),
            (Integer::Int64(_), Integer::Int16(_)) => todo!(),
            (Integer::Int64(_), Integer::UInt16(_)) => todo!(),
            (Integer::Int64(_), Integer::Int32(_)) => todo!(),
            (Integer::Int64(_), Integer::UInt32(_)) => todo!(),
            (Integer::Int64(_), Integer::Int64(_)) => todo!(),
            (Integer::Int64(_), Integer::UInt64(_)) => todo!(),
            (Integer::Int64(_), Integer::BigInt(_)) => todo!(),
            (Integer::Int64(_), Integer::BigUint(_)) => todo!(),
            (Integer::Int64(_), Integer::Inf) => todo!(),
            (Integer::UInt64(_), Integer::Int8(_)) => todo!(),
            (Integer::UInt64(_), Integer::UInt8(_)) => todo!(),
            (Integer::UInt64(_), Integer::Int16(_)) => todo!(),
            (Integer::UInt64(_), Integer::UInt16(_)) => todo!(),
            (Integer::UInt64(_), Integer::Int32(_)) => todo!(),
            (Integer::UInt64(_), Integer::UInt32(_)) => todo!(),
            (Integer::UInt64(_), Integer::Int64(_)) => todo!(),
            (Integer::UInt64(_), Integer::UInt64(_)) => todo!(),
            (Integer::UInt64(_), Integer::BigInt(_)) => todo!(),
            (Integer::UInt64(_), Integer::BigUint(_)) => todo!(),
            (Integer::UInt64(_), Integer::Inf) => todo!(),
            (Integer::BigInt(_), Integer::Int8(_)) => todo!(),
            (Integer::BigInt(_), Integer::UInt8(_)) => todo!(),
            (Integer::BigInt(_), Integer::Int16(_)) => todo!(),
            (Integer::BigInt(_), Integer::UInt16(_)) => todo!(),
            (Integer::BigInt(_), Integer::Int32(_)) => todo!(),
            (Integer::BigInt(_), Integer::UInt32(_)) => todo!(),
            (Integer::BigInt(_), Integer::Int64(_)) => todo!(),
            (Integer::BigInt(_), Integer::UInt64(_)) => todo!(),
            (Integer::BigInt(_), Integer::BigInt(_)) => todo!(),
            (Integer::BigInt(_), Integer::BigUint(_)) => todo!(),
            (Integer::BigInt(_), Integer::Inf) => todo!(),
            (Integer::BigUint(_), Integer::Int8(_)) => todo!(),
            (Integer::BigUint(_), Integer::UInt8(_)) => todo!(),
            (Integer::BigUint(_), Integer::Int16(_)) => todo!(),
            (Integer::BigUint(_), Integer::UInt16(_)) => todo!(),
            (Integer::BigUint(_), Integer::Int32(_)) => todo!(),
            (Integer::BigUint(_), Integer::UInt32(_)) => todo!(),
            (Integer::BigUint(_), Integer::Int64(_)) => todo!(),
            (Integer::BigUint(_), Integer::UInt64(_)) => todo!(),
            (Integer::BigUint(_), Integer::BigInt(_)) => todo!(),
            (Integer::BigUint(_), Integer::BigUint(_)) => todo!(),
            (Integer::BigUint(_), Integer::Inf) => todo!(),
            (Integer::Inf, Integer::Int8(_)) => Integer::Inf,
            (Integer::Inf, Integer::UInt8(_)) => Integer::Inf,
            (Integer::Inf, Integer::Int16(_)) => Integer::Inf,
            (Integer::Inf, Integer::UInt16(_)) => Integer::Inf,
            (Integer::Inf, Integer::Int32(_)) => Integer::Inf,
            (Integer::Inf, Integer::UInt32(_)) => Integer::Inf,
            (Integer::Inf, Integer::Int64(_)) => Integer::Inf,
            (Integer::Inf, Integer::UInt64(_)) => Integer::Inf,
            (Integer::Inf, Integer::BigInt(_)) => Integer::Inf,
            (Integer::Inf, Integer::BigUint(_)) => Integer::Inf,
            (Integer::Inf, Integer::Inf) => Integer::Inf,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Int8(pub i8);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UInt8(pub u8);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Int16(pub i16);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UInt16(pub u16);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Int32(pub i32);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UInt32(pub u32);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Int64(pub i64);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UInt64(pub u64);

impl Add for Int8 {
    type Output = Integer;

    fn add(self, rhs: Self) -> Self::Output {
        if let Some(sum) = self.0.checked_add(rhs.0) {
            return Integer::Int8(Self(sum));
        } else {
            return Integer::Int16(Int16(self.0 as i16 + rhs.0 as i16));
        }
    }
}

impl Sub for Int8 {
    type Output = Integer;

    fn sub(self, rhs: Self) -> Self::Output {
        if let Some(diff) = self.0.checked_sub(rhs.0) {
            return Integer::Int8(Self(diff));
        } else {
            return Integer::Int16(Int16(self.0 as i16 - rhs.0 as i16));
        }
    }
}

impl Mul for Int8 {
    type Output = Integer;

    fn mul(self, rhs: Self) -> Self::Output {
        if let Some(prod) = self.0.checked_mul(rhs.0) {
            return Integer::Int8(Self(prod));
        } else if let Some(prod) = (self.0 as i16).checked_mul(rhs.0 as i16) {
            return Integer::Int16(Int16(prod));
        } else {
            return Integer::Int32(Int32(self.0 as i32 * rhs.0 as i32));
        }
    }
}

impl Div for Int8 {
    type Output = Integer;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.0 == 0 {
            return Integer::Inf;
        } else if rhs.0 == -1 {
            return Integer::Int16(Int16(self.0 as i16 / rhs.0 as i16));
        } else {
            return Integer::Int8(Int8(self.0 / rhs.0));
        }
    }
}

impl Rem for Int8 {
    type Output = Integer;

    fn rem(self, rhs: Self) -> Self::Output {
        if rhs.0 == 0 {
            return Integer::Inf;
        } else {
            return Integer::Int8(Int8(self.0 % rhs.0));
        }
    }
}

impl Add for Int16 {
    type Output = Integer;

    fn add(self, rhs: Self) -> Self::Output {
        if let Some(sum) = self.0.checked_add(rhs.0) {
            return Integer::Int16(Self(sum));
        } else {
            return Integer::Int32(Int32(self.0 as i32 + rhs.0 as i32));
        }
    }
}
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Sign {
//     Plus,
//     Minus,
//     None,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct BigInt {
//     sign: Sign,
//     data: BigUint,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct BigUint(pub Vec<u64>);

// impl BigUint {
//     #[inline]
//     pub fn to_str_radix(&self, radix: u32) -> String {
//         let mut v = to_str_radix_reversed(self, radix);
//         v.reverse();
//         unsafe { String::from_utf8_unchecked(v) }
//     }
// }

// impl Debug for BigUint {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("BigUint")
//     }
// }

// impl Display for BigUint {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.pad_integral(true, "0x", &self.to_str_radix(10))
//     }
// }
