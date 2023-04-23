use super::{opcode::OpCode, value::Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: OpCode,
    pub operand: Option<Box<Value>>,
}
