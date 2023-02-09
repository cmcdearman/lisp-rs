pub mod integer;
pub mod rational;
pub mod float;
pub mod complex;

use std::fmt::Display;

use num_complex::Complex64;

use self::{
    integer::Integer,
    rational::Rational, float::Float,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub enum Number {
    Integer(Integer),
    Float(Float),
    Rational(Rational),
    Complex(Complex64)
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Integer(n) => write!(f, "{}", n),
            Number::Float(n) => write!(f, "{}", n),
            Number::Rational(n) => write!(f, "{}", n),
            Number::Complex(_) => todo!(),
        }
    }
}