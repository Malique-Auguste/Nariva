use crate::instruction::{Opcode, decode, lower_8_bits};

struct Machine {
    program: Vec<u32>,
    program_address: usize,

    stack: Vec<u32>,
    registers: [u32; 32],

    zero_flag: bool,
    sign_flag: bool
}

impl Machine {
    pub fn new(program: Vec<u32>) -> Machine {
        Machine {
            program,
            program_address: 0,
            stack: Vec::new(),
            registers: [0; 32],
            zero_flag: true,
            sign_flag: true,
        }
    }

    pub fn execute_instruction(&mut self) {
        let (opcode, operand1, operand2) = decode(self.program[self.program_address]);

        match opcode {
            Opcode::Illegal => unimplemented!(),
            Opcode::Halt => unimplemented!(),

            Opcode::Push => self.stack.push(operand1 as u32),
            Opcode::PushR => self.stack.push(self.registers[lower_8_bits(operand1) as usize]),

            Opcode::Pop => {self.stack.pop();},
            Opcode::PopR => self.registers[lower_8_bits(operand1) as usize] = match self.stack.pop(){
                Some(v) => v,
                None => unimplemented!(),
            },
        }
    }
}