use std::{fmt::Display, rc::Rc, cell::RefCell};

use crate::parser::sexpr::{env::Env, symbol::Symbol};

pub mod tests;

pub struct VM {
    registers: [i64; 64],
    pc: usize,
    program: Vec<u8>,
    heap: Vec<u8>,
    stack: [i64; 64]
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 64],
            pc: 0,
            program: Vec::new(),
            heap: Vec::new(),
            stack: [0; 64],
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.pc >= self.program.len() {
                break;
            }
            match self.decode_opcode() {
                Opcode::LoadConst => {
                    let register = self.next_8_bits() as usize;
                    let number = self.next_16_bits() as u16;
                    self.registers[register] = number as i64;
                    continue;
                }
                Opcode::LoadVar => {
                    let register = self.next_8_bits() as usize;
                    let number = self.next_16_bits() as u16;
                    self.registers[register] = number as i64;
                    continue;
                }
                Opcode::StoreVar => {
                    // let register = self.next_8_bits() as usize;
                    // let var_name = self.next_string();
                    // let var_value = match self.env.as_ref().borrow().find(&Symbol::from(&*var_name)) {
                    //     Some(value) => value,
                    //     None => panic!("Undefined variable: {}", var_name),
                    // };
                    // self.registers[register] = var_value;
                    continue;
                }
                Opcode::Add => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 + register2;
                }
                Opcode::Sub => todo!(),
                Opcode::Mul => todo!(),
                Opcode::Div => todo!(),
                Opcode::Eq => todo!(),
                Opcode::Neq => todo!(),
                Opcode::Jump => todo!(),
                Opcode::Jeq => todo!(),
                Opcode::Return => todo!(),
                Opcode::Halt => {
                    println!("Halt encountered");
                    return;
                }
                _ => todo!(),
            }
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }

    fn next_string(&mut self) -> String {
        let len = self.next_8_bits() as usize;
        let bytes = self.next_n_bytes(len);
        String::from_utf8(bytes).expect("Invalid string")
    }

    fn next_n_bytes(&mut self, n: usize) -> Vec<u8> {
        // let mut bytes = Vec::new();
        // for i in 1..n {
        //     bytes.push(self.next_8_bits());
        // }
        // bytes
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Halt,
    Nop,
    LoadConst,
    LoadVar,
    StoreVar,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Not,
    Neq,
    Jump,
    Jeq,
    Return,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::Halt,
            1 => Opcode::Nop,
            2 => Opcode::LoadConst,
            3 => Opcode::LoadVar,
            4 => Opcode::StoreVar,
            5 => Opcode::Add,
            6 => Opcode::Sub,
            7 => Opcode::Mul,
            8 => Opcode::Div,
            9 => Opcode::Eq,
            10 => Opcode::Not,
            11 => Opcode::Neq,
            12 => Opcode::Jump,
            13 => Opcode::Jeq,
            14 => Opcode::Return,
            _ => Opcode::Halt,
        }
    }
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> Self {
        match op {
            Opcode::Halt => 0,
            Opcode::Nop => 1,
            Opcode::LoadConst => 2,
            Opcode::LoadVar => 3,
            Opcode::StoreVar => 4,
            Opcode::Add => 5,
            Opcode::Sub => 6,
            Opcode::Mul => 7,
            Opcode::Div => 8,
            Opcode::Eq => 9,
            Opcode::Not => 10,
            Opcode::Neq => 11,
            Opcode::Jump => 12,
            Opcode::Jeq => 13,
            Opcode::Return => 14,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}
