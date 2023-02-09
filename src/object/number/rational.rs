use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub}, iter::Sum,
};

use super::integer::Integer;


#[derive(Debug, Clone)]
pub struct Rational {
    numerator: Integer,
    denominator: Integer,
}

impl Rational {
    pub fn new(numerator: Integer, denominator: Integer) -> Self {
        Self {
            numerator,
            denominator,
        }.normalize()
    }

    pub fn normalize(&self) -> Self {
        todo!()
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        }
        .normalize()
    }
}

// impl Add<Integer> for Rational {
//     type Output = Rational;

//     fn add(self, rhs: Integer) -> Self::Output {
//         self + Rational::from(rhs)
//     }
// }

// impl Add<i64> for Rational {
//     type Output = Rational;

//     fn add(self, rhs: i64) -> Self::Output {
//         self + Rational::from(rhs)
//     }
// }

// impl Add<Rational> for i64 {
//     type Output = Rational;

//     fn add(self, rhs: Rational) -> Self::Output {
//         Rational::from(self) + rhs
//     }
// }

// impl Add<f64> for Rational {
//     type Output = f64;

//     fn add(self, rhs: f64) -> Self::Output {
//         f64::from(self) + rhs
//     }
// }

// impl Add<Rational> for f64 {
//     type Output = f64;

//     fn add(self, rhs: Rational) -> Self::Output {
//         self + f64::from(rhs)
//     }
// }

// impl Sub for Rational {
//     type Output = Self;

//     fn sub(self, rhs: Self) -> Self::Output {
//         Self {
//             numerator: self.numerator * rhs.denominator - rhs.numerator * self.denominator,
//             denominator: self.denominator * rhs.denominator,
//         }
//         .normalize()
//     }
// }

// impl Mul for Rational {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Self::Output {
//         Self {
//             numerator: self.numerator * rhs.numerator,
//             denominator: self.denominator * rhs.denominator,
//         }
//         .normalize()
//     }
// }

// impl Div for Rational {
//     type Output = Self;

//     fn div(self, rhs: Self) -> Self::Output {
//         Self {
//             numerator: self.numerator * rhs.denominator,
//             denominator: self.denominator * rhs.numerator,
//         }
//     }
// }

// impl Sum for Rational {
//     fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
//         iter.fold(Rational::from(FixNum::ZERO), |r1, r2| r1 + r2)
//     }
// }

// impl From<i64> for Rational {
//     fn from(value: i64) -> Self {
//         Self {
//             numerator: Integer::FixNum(FixNum(value)),
//             denominator: FixNum::ONE,
//         }
//     }
// }

// impl From<Integer> for Rational {
//     fn from(value: Integer) -> Self {
//         Self {
//             numerator: value,
//             denominator: Integer::FixNum(FixNum::from(1)),
//         }
//     }
// }

// impl From<FixNum> for Rational {
//     fn from(value: FixNum) -> Self {
//         Self {
//             numerator: Integer::FixNum(value),
//             denominator: Integer::FixNum(FixNum::from(1)),
//         }
//     }
// }

// impl From<BigNum> for Rational {
//     fn from(value: BigNum) -> Self {
//         Self {
//             numerator: Integer::BigNum(value),
//             denominator: Integer::FixNum(FixNum::from(1)),
//         }
//     }
// }

// impl From<Rational> for f64 {
//     fn from(value: Rational) -> Self {
//         value.numerator / 
//     }
// }

// // impl From<f64> for Rational {
// //     fn from(value: f64) -> Self {
// //         Self { numerator: , denominator: () }
// //     }
// // }