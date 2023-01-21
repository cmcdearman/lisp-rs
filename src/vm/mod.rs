pub struct VirtualMachine {
    registers: [i32; 8],
    counter: usize,
}

pub enum OpCode {
    LoadConst(usize, i32),
    Add(usize, usize, usize),
    Jump(usize),
    Return,
    Halt,
}

fn execute_instruction(vm: &mut VirtualMachine, program: &[OpCode]) {
    let instruction = program[vm.program_counter];
    vm.program_counter += 1;
    match instruction {
        OpCode::LoadConst(register, value) => {
            vm.registers[register] = value;
        }
        OpCode::Add(dest, src1, src2) => {
            vm.registers[dest] = vm.registers[src1] + vm.registers[src2];
        }
        OpCode::Jump(location) => {
            vm.program_counter = location;
        }
        OpCode::Halt => {
            println!("Program halted");
            return;
        }
    }
}
