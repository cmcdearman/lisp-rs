use self::{opcode::OpCode, runtime_error::Result, value::Value};

pub struct VM {
    pc: usize,
    program: Vec<u8>,
    stack: Vec<Value>,
    heap: Vec<Value>,
}

pub mod instruction;
pub mod opcode;
pub mod runtime_error;
pub mod value;

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
