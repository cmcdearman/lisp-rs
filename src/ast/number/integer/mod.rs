use self::{fixnum::FixNum, bignum::BigNum};

pub mod fixnum;
pub mod bignum;

#[derive(Debug, Clone)]
pub enum Integer {
    FixNum(FixNum),
    BigNum(BigNum),
}