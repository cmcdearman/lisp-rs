pub mod integer;
pub mod rational;
pub mod float;
pub mod complex;

use std::{fmt::Display};

use num_complex::Complex64;
use num_rational::Rational64;

use self::{
    integer::Integer,
    float::Float,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseNumberError(pub String);

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(Integer),
    Float(Float),
    Rational(Rational64),
    Complex(Complex64)
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Integer(n) => write!(f, "{}", n),
            Number::Float(n) => write!(f, "{}", n),
            Number::Rational(n) => write!(f, "{}", n),
            Number::Complex(n) => write!(f, "{}", n),
        }
    }
}