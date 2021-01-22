use crate::instruction::{Opcode, ByteOps};

pub const HEADER: [u8; 17] = [78, 97, 114, 105, 118, 97, 32, 69, 120, 101, 99, 117, 116, 97, 98, 108, 101];

pub struct Machine {
    pub program: Vec<u8>,
    pub program_address: usize,

    pub stack: Vec<u32>,
    registers: [u32; 32],

    zero_flag: bool,
    sign_flag: bool
}

impl Machine {
    pub fn new(program: Vec<u8>) -> Machine {
        Machine {
            program,
            program_address: 0,
            stack: Vec::new(),
            registers: [0; 32],
            zero_flag: true,
            sign_flag: true,
        }
    }

    pub fn next_8_bits(&mut self) -> u8 {
        self.program_address += 1;
        self.program[self.program_address]
    }

    pub fn next_16_bits(&mut self) -> u16 {
        self.program_address += 2;
        (self.program[self.program_address - 1] as u16) << 8 | self.program[self.program_address] as u16
    }

    pub fn run(&mut self) -> u32 {
        if self.is_nariva_file() {
            self.program_address = HEADER.len() - 1;
            loop {
                self.execute_instruction();

                if self.program_address >= self.program.len() - 1 {
                    return self.stack[self.stack.len() - 1]
                }
            }
        }
        else {
            u32::MAX
        }
    }

    pub fn execute_instruction(&mut self) {
        let opcode = self.next_8_bits().into();
        match opcode {
            Opcode::Illegal => unimplemented!("program address: {}, {:?}, {:?}", self.program_address, self.stack, self.registers),
            Opcode::Halt => unimplemented!("program address: {}, {:?}, {:?}", self.program_address, self.stack, self.registers),

            Opcode::Push => {
                if self.next_8_bits() == 0 {
                    let num = self.next_16_bits() as u32;
                    self.stack.push(num);
                }
                else {
                    let index = self.next_8_bits() as usize;
                    self.stack.push(self.registers[index]);
                }
            },

            Opcode::Pop => {
                if self.next_8_bits() == 0 {
                    self.stack.pop();
                }
                else {
                    self.registers[self.next_8_bits() as usize] = match self.stack.pop(){
                        Some(v) => v,
                        None => unimplemented!(),
                    };
                }
            },

            Opcode::Add32 => {
                let [num1, num2] = self.pop_2_u32();
                let option = self.next_8_bits();
                if option == 0 {
                    self.stack.push(num2 + num1)
                }
                else if option == 1 {
                    self.stack.push((num2 as i32 + num1 as i32) as u32)
                }
                else {
                    self.stack.push((num2 as f32 + num1 as f32) as u32)
                }
            },
            Opcode::Sub32 => {
                let [num1, num2] = self.pop_2_u32();
                let option = self.next_8_bits();
                if option == 0 {
                    self.stack.push(num2 - num1)
                }
                else if option == 1 {
                    self.stack.push((num2 as i32 - num1 as i32) as u32)
                }
                else {
                    self.stack.push((num2 as f32 - num1 as f32) as u32)
                }
            },
            Opcode::Mul32 => {
                let [num1, num2] = self.pop_2_u32();
                let option = self.next_8_bits();
                if option == 0 {
                    self.stack.push(num2 * num1)
                }
                else if option == 1 {
                    self.stack.push((num2 as i32 * num1 as i32) as u32)
                }
                else {
                    self.stack.push((num2 as f32 * num1 as f32) as u32)
                }
            },
            Opcode::Div32 => {
                let [num1, num2] = self.pop_2_u32();
                let option = self.next_8_bits();
                if option == 0 {
                    self.stack.push(num2 / num1)
                }
                else if option == 1 {
                    self.stack.push((num2 as i32 / num1 as i32) as u32)
                }
                else {
                    self.stack.push((num2 as f32 / num1 as f32) as u32)
                }
            },

            Opcode::Add64 => {
                let [num1, num2] = self.pop_2_u64();
                let option = self.next_8_bits();
                if option == 0 {
                    self.stack.append(&mut ((num2 + num1).split_smaller().into()));
                }
                else if option == 1 {
                    self.stack.append(&mut (((num2 as i64 + num1 as i64) as u64).split_smaller().into()));
                }
                else {
                    self.stack.append(&mut (((num2 as f64 + num1 as f64) as u64).split_smaller().into()));
                }
            },
            Opcode::Sub64 => {
                let [num1, num2] = self.pop_2_u64();
                let option = self.next_8_bits();
                if option == 0 {
                    self.stack.append(&mut ((num2 - num1).split_smaller().into()));
                }
                else if option == 1 {
                    self.stack.append(&mut (((num2 as i64 - num1 as i64) as u64).split_smaller().into()));
                }
                else {
                    self.stack.append(&mut (((num2 as f64 - num1 as f64) as u64).split_smaller().into()));
                }
            },
            Opcode::Mul64 => {
                let [num1, num2] = self.pop_2_u64();
                let option = self.next_8_bits();
                if option == 0 {
                    self.stack.append(&mut ((num2 * num1).split_smaller().into()));
                }
                else if option == 1 {
                    self.stack.append(&mut (((num2 as i64 * num1 as i64) as u64).split_smaller().into()));
                }
                else {
                    self.stack.append(&mut (((num2 as f64 * num1 as f64) as u64).split_smaller().into()));
                }
            },
            Opcode::Div64 => {
                let [num1, num2] = self.pop_2_u64();
                let option = self.next_8_bits();
                if option == 0 {
                    self.stack.append(&mut ((num2 / num1).split_smaller().into()));
                }
                else if option == 1 {
                    self.stack.append(&mut (((num2 as i64 / num1 as i64) as u64).split_smaller().into()));
                }
                else {
                    self.stack.append(&mut (((num2 as f64 / num1 as f64) as u64).split_smaller().into()));
                }
            },

            Opcode::Shift => {
                if self.next_8_bits() == 0 {
                    let [index1, index2] = [self.next_8_bits(), self.next_8_bits()];
                    self.registers[index1 as usize] = self.registers[index1 as usize] << self.registers[index2 as usize];
                }
                else {
                    let [index, shift_amount] = [self.next_8_bits(), self.next_8_bits()];
                    self.registers[index as usize] = self.registers[index as usize] << shift_amount;
                }
            },
            Opcode::BitAnd => {
                let [index1, index2] = [self.next_8_bits(), self.next_8_bits()];
                self.registers[index1 as usize] = self.registers[index1 as usize] & self.registers[index2 as usize];
            },
            Opcode::BitOr => {
                let [index1, index2] = [self.next_8_bits(), self.next_8_bits()];
                self.registers[index1 as usize] = self.registers[index1 as usize] | self.registers[index2 as usize];
            },
            Opcode::BitXor => {
                let [index1, index2] = [self.next_8_bits(), self.next_8_bits()];
                self.registers[index1 as usize] = self.registers[index1 as usize] ^ self.registers[index2 as usize];
            },
            Opcode::BitNot => {
                let [index1, index2] = [self.next_8_bits(), self.next_8_bits()];
                self.registers[index1 as usize] = !self.registers[index2 as usize];
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

    pub fn is_nariva_file(&self) -> bool {
        if self.program.len() <= HEADER.len() {
            return false
        }
        else {
            if self.program[0..HEADER.len()] == HEADER {
                return true
            }
            else {
                return false
            }
        }
    }
}