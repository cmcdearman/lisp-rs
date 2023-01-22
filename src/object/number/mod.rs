pub mod integer;
pub mod rational;

use std::fmt::Display;

use self::{
    integer::{fixnum::FixNum, Integer},
    rational::Rational,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub enum Number {
    Integer(Integer),
    Float(f64),
    Rational(Rational),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Integer(n) => write!(f, "{}", n),
            Number::Float(n) => write!(f, "{}", n),
            Number::Rational(n) => write!(f, "{}", n),
        }
    }
}

pub fn try_add_nums(n1: Number, n2: Number) -> Result<Number, String> {
    match (n1, n2) {
        (Number::Integer(l), Number::Integer(r)) => Ok(Number::Integer(l + r)),
        (Number::Integer(l), Number::Float(r)) => match l {
            Integer::FixNum(n) => Ok(Number::Float(n + r)),
            _ => Err("cannot add float to bignum".to_string()),
        },
        (Number::Integer(l), Number::Rational(r)) => Ok(Number::Rational(l + r)),
        (Number::Float(l), Number::Integer(r)) => match r {
            Integer::FixNum(n) => Ok(Number::Float(l + n)),
            _ => Err("cannot add float to bignum".to_string()),
        },
        (Number::Float(l), Number::Float(r)) => Ok(Number::Float(l + r)),
        (Number::Float(l), Number::Rational(r)) => Ok(Number::Rational(l + r)),
        (Number::Rational(l), Number::Integer(r)) => Ok(Number::Rational(l + r)),
        (Number::Rational(l), Number::Float(r)) => Ok(Number::Rational(l + r)),
        (Number::Rational(l), Number::Rational(r)) => Ok(Number::Rational(l + r)),
    }
}
