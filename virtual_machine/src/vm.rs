use crate::instruction::{Opcode, ByteOps};

pub const HEADER: [u8; 17] = [78, 97, 114, 105, 118, 97, 32, 69, 120, 101, 99, 117, 116, 97, 98, 108, 101];

pub struct Machine {
    pub program: Vec<u8>,
    pub program_address: usize,

    pub stack: Vec<u64>,

    zero_flag: bool,
    sign_flag: bool
}

impl Machine {
    pub fn new(program: Vec<u8>) -> Machine {
        Machine {
            program,
            program_address: 0,
            stack: Vec::new(),
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

    pub fn run(&mut self) -> u64 {
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
            u64::MAX
        }
    }

    pub fn execute_instruction(&mut self) {
        let opcode = self.next_8_bits().into();
        println!("{:?}, {}", opcode, self.program_address);
        match opcode {
            Opcode::Illegal => unimplemented!("program address: {}, {:?}", self.program_address, self.stack),
            Opcode::Halt => unimplemented!("program address: {}, {:?}", self.program_address, self.stack),

            Opcode::Push => {
                let option = self.next_8_bits();
                if option == 0 {
                    //pushes u8
                    let num = self.next_8_bits() as u64;
                    self.stack.push(num);
                }
                else if option == 1 {
                    //pushes u16
                    let num = self.next_16_bits() as u64;
                    self.stack.push(num);
                }
                else if option == 2 {
                    //pushes u32
                    let num = (self.next_16_bits() as u64) << 16 | (self.next_16_bits() as u64);
                    self.stack.push(num);
                }
                else {
                    //pushes u64
                    let num = (self.next_16_bits() as u64) << 48 | (self.next_16_bits() as u64) << 32 | (self.next_16_bits() as u64) << 16 | self.next_16_bits() as u64;
                    self.stack.push(num);
                }
            },

            Opcode::Pop => {
                self.stack.pop();
            },

            Opcode::AddU => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 + num1)
            },
            Opcode::SubU => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 - num1)
            },
            Opcode::MulU => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 * num1)
            },
            Opcode::DivU => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 / num1)
            },

            Opcode::AddI => {
                let [num1, num2] = self.double_pop();
                let result = i64::from_be_bytes(num2.to_be_bytes()) + i64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },
            Opcode::SubI => {
                let [num1, num2] = self.double_pop();
                let result = i64::from_be_bytes(num2.to_be_bytes()) - i64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },
            Opcode::MulI => {
                let [num1, num2] = self.double_pop();
                let result = i64::from_be_bytes(num2.to_be_bytes()) * i64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },
            Opcode::DivI => {
                let [num1, num2] = self.double_pop();
                let result = i64::from_be_bytes(num2.to_be_bytes()) / i64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },

            Opcode::AddF => {
                let [num1, num2] = self.double_pop();
                let result = f64::from_be_bytes(num2.to_be_bytes()) + f64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },
            Opcode::SubF => {
                let [num1, num2] = self.double_pop();
                let result = f64::from_be_bytes(num2.to_be_bytes()) - f64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },
            Opcode::MulF => {
                let [num1, num2] = self.double_pop();
                let result = f64::from_be_bytes(num2.to_be_bytes()) * f64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },
            Opcode::DivF => {
                let [num1, num2] = self.double_pop();
                let result = f64::from_be_bytes(num2.to_be_bytes()) / f64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },

            Opcode::Shift => {
                let [num1, num2] = self.double_pop();
                if self.next_8_bits() == 0 {
                    self.stack.push(num2 << num1)
                }
                else {
                    self.stack.push(num2 >> num1)
                }
            },
            Opcode::BitAnd => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 & num1);
            },
            Opcode::BitOr => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 | num1);
            },
            Opcode::BitXor => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 ^ num1);
            },
            Opcode::BitNot => {
                let num = match self.stack.pop() {
                    Some(x) => x,
                    None => unimplemented!()
                };
                self.stack.push(!num)
            }
        }
    }

    pub fn double_pop(&mut self) -> [u64; 2] {
        [
            match self.stack.pop() {
                Some(x) => x,
                None => unimplemented!()
            },
            match self.stack.pop() {
                Some(y) => y,
                None => unimplemented!()
            }
        ]
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