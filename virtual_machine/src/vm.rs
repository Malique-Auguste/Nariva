use crate::instruction::{Opcode, decode, ByteOps};

pub struct Machine {
    pub program: Vec<u32>,
    pub program_address: usize,

    pub stack: Vec<u32>,
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

    pub fn run(&mut self) -> u32 {
        loop {
            self.execute_instruction();
            self.program_address += 1;

            if self.program_address == self.program.len() {
                break self.stack[self.stack.len() - 1]
            }
        }
    }

    pub fn execute_instruction(&mut self) {
        let (opcode, operand1, operand2, operand3) = decode(self.program[self.program_address]);

        match opcode {
            Opcode::Illegal => unimplemented!(),
            Opcode::Halt => unimplemented!(),

            Opcode::Push => self.stack.push(u16::from_be_bytes([operand1, operand2]) as u32),
            Opcode::PushR => self.stack.push(self.registers[operand1 as usize]),

            Opcode::Pop => {self.stack.pop();},
            Opcode::PopR => self.registers[operand1 as usize] = match self.stack.pop(){
                Some(v) => v,
                None => unimplemented!(),
            },

            Opcode::Add32 => {
                let [num1, num2] = self.pop_2_u32();
                if operand3 == 0 {
                    self.stack.push(num2 + num1)
                }
                else if operand3 == 1 {
                    self.stack.push((num2 as i32 + num1 as i32) as u32)
                }
                else {
                    self.stack.push((num2 as f32 + num1 as f32) as u32)
                }
            },

            Opcode::Sub32 => {
                let [num1, num2] = self.pop_2_u32();
                if operand3 == 0 {
                    self.stack.push(num2 - num1)
                }
                else if operand3 == 1 {
                    self.stack.push((num2 as i32 - num1 as i32) as u32)
                }
                else {
                    self.stack.push((num2 as f32 - num1 as f32) as u32)
                }
            },

            Opcode::Mul32 => {
                let [num1, num2] = self.pop_2_u32();
                if operand3 == 0 {
                    self.stack.push(num2 - num1)
                }
                else if operand3 == 1 {
                    self.stack.push((num2 as i32 - num1 as i32) as u32)
                }
                else {
                    self.stack.push((num2 as f32 - num1 as f32) as u32)
                }
            },

            Opcode::Div32 => {
                let [num1, num2] = self.pop_2_u32();
                if operand3 == 0 {
                    self.stack.push(num2 / num1)
                }
                else if operand3 == 1 {
                    self.stack.push((num2 as i32 / num1 as i32) as u32)
                }
                else {
                    self.stack.push((num2 as f32 / num1 as f32) as u32)
                }
            },

            Opcode::Add64 => {
                let [num1, num2] = self.pop_2_u64();
                if operand3 == 0 {
                    self.stack.append(&mut (((num2 as i64 + num1 as i64) as u64).split_smaller().into()));
                }
                else if operand3 == 1 {
                    self.stack.append(&mut (((num2 as i64 + num1 as i64) as u64).split_smaller().into()));
                }
                else {
                    self.stack.append(&mut (((num2 as f64 + num1 as f64) as u64).split_smaller().into()));
                }
                
            },

            Opcode::Sub64 => {
                let [num1, num2] = self.pop_2_u64();
                if operand3 == 0 {
                    self.stack.append(&mut ((num2 - num1).split_smaller().into()));
                }
                else if operand3 == 1 {
                    self.stack.append(&mut (((num2 as i64 - num1 as i64) as u64).split_smaller().into()));
                }
                else {
                    self.stack.append(&mut (((num2 as f64 - num1 as f64) as u64).split_smaller().into()));
                }
            },

            Opcode::Mul64 => {
                let [num1, num2] = self.pop_2_u64();
                if operand3 == 0 {
                    self.stack.append(&mut ((num2 * num1).split_smaller().into()));
                }
                else if operand3 == 1 {
                    self.stack.append(&mut (((num2 as i64 * num1 as i64) as u64).split_smaller().into()));
                }
                else {
                    self.stack.append(&mut (((num2 as f64 * num1 as f64) as u64).split_smaller().into()));
                }
            },

            Opcode::Div64 => {
                let [num1, num2] = self.pop_2_u64();
                if operand3 == 0 {
                    self.stack.append(&mut ((num2 / num1 ).split_smaller().into()));
                }
                else if operand3 == 1 {
                    self.stack.append(&mut (((num2 as i64 / num1 as i64) as u64).split_smaller().into()));
                }
                else {
                    self.stack.append(&mut (((num2 as f64 / num1 as f64) as u64).split_smaller().into()));
                }
            }
        }
    }

    pub fn pop_2_u32(&mut self) -> [u32; 2] {
        [
            match self.stack.pop() {
                Some(x) => x,
                None => unimplemented!()
            },
            match self.stack.pop() {
                Some(x) => x,
                None => unimplemented!()
            }
        ]
    }

    pub fn pop_2_u64(&mut self) -> [u64; 2] {
        let num1 = {
            (match self.stack.pop() {
                Some(x) => x as u64,
                None => unimplemented!()
            })
            |
            (match self.stack.pop() {
                Some(x) => (x as u64) << 32,
                None => unimplemented!()
            })
        };

        let num2 = {
            (match self.stack.pop() {
                Some(x) => x as u64,
                None => unimplemented!()
            })
            |
            (match self.stack.pop() {
                Some(x) => (x as u64) << 32,
                None => unimplemented!()
            })
        };

        [num1, num2]
    }
}