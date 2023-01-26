#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Integer {
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    BigInt,
    BigUint,
}

pub struct Int8(pub i8);
pub struct UInt8(pub u8);
pub struct Int16(pub i16);
pub struct UInt16(pub u16);
pub struct Int32(pub i32);
pub struct UInt32(pub u32);
pub struct Int64(pub i64);
pub struct UInt64(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sign {
    Plus,
    Minus,
    None,
}

pub struct BigInt {
    sign: Sign,
    data: BigUint,
}
pub struct BigUint(pub Vec<u64>);