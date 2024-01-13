use std::str::FromStr;

use num_complex::Complex64;
use num_rational::Rational64;

#[derive(Debug, Clone, PartialEq)]
pub enum Num {
    Int(i64),
    Float(f64),
    Rational(Rational),
    Complex(Complex64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rational(Rational64);

impl Rational {
    pub fn new(numer: i64, denom: i64) -> Self {
        Self(Rational64::new(numer, denom))
    }

    pub fn numer(&self) -> i64 {
        *self.0.numer()
    }

    pub fn denom(&self) -> i64 {
        *self.0.denom()
    }
}

impl FromStr for Rational {
    type Err = num_rational::ParseRatioError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
