use std::{fmt::{Debug, Display}, ops::Add};

use num_bigint::{BigInt, BigUint};

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
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Add for Integer {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Integer::Int8(_) => todo!(),
            Integer::UInt8(_) => todo!(),
            Integer::Int16(_) => todo!(),
            Integer::UInt16(_) => todo!(),
            Integer::Int32(_) => todo!(),
            Integer::UInt32(_) => todo!(),
            Integer::Int64(_) => todo!(),
            Integer::UInt64(_) => todo!(),
            Integer::BigInt(_) => todo!(),
            Integer::BigUint(_) => todo!(),
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


