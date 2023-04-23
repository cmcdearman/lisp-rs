#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    LoadConst,
    LoadVar,
    Add,
    Sub,
    Mul,
    Div,
    Halt,
}

impl OpCode {}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => OpCode::LoadConst,
            1 => OpCode::Add,
            2 => OpCode::Sub,
            3 => OpCode::Mul,
            4 => OpCode::Div,
            5 => OpCode::Halt,
            _ => panic!("Invalid opcode"),
        }
    }
}
