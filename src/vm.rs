use crate::list::List;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Symbol(String),
    List(List<Value>),
    Nil,
}

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

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: OpCode,
    pub operand: Option<Box<Value>>,
}

pub struct VM {
    pc: usize,
    program: Vec<u8>,
    stack: Vec<Value>,
    heap: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeError(pub String);

impl RuntimeError {
    pub fn new(msg: String) -> RuntimeError {
        RuntimeError(msg)
    }
}

impl From<&str> for RuntimeError {
    fn from(msg: &str) -> RuntimeError {
        RuntimeError::new(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

impl VM {
    pub fn new(program: Vec<u8>) -> VM {
        VM {
            pc: 0,
            program,
            stack: Vec::new(),
            heap: Vec::new(),
        }
    }

    pub fn run(&self) -> Option<Value> {
        while self.pc < self.program.len() {
            let opcode = self.read_opcode();
            match opcode {
                OpCode::Halt => None,
                _ => {
                    panic!("Unimplemented opcode: {:?}", opcode)
                }
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.program[self.pc];
        self.pc += 1;
        byte
    }

    fn read_opcode(&mut self) -> OpCode {
        let opcode = self.read_byte();
        OpCode::from(opcode)
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Result<Value> {
        self.stack.pop().ok_or("Stack underflow".into())
    }
}
