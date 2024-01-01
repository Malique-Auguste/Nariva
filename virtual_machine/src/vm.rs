use crate::instruction::{OpCode, ByteOps};

//Code at the start of all programs to ensure that they are nariva files.
//The numebrs decode to "Nariva Executable"
pub const HEADER: [u8; 17] = [78, 97, 114, 105, 118, 97, 32, 69, 120, 101, 99, 117, 116, 97, 98, 108, 101];

pub struct Machine {
    pub program: Vec<u8>,
    pub program_address: usize,

    pub stack: Vec<u64>,

    show: bool,
    zero_flag: bool,
    sign_flag: bool
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            program: Vec::new(),
            program_address: 0,
            stack: Vec::new(),
            show: false,
            zero_flag: true,
            sign_flag: true,
        }
    }

    pub fn next_8_bits(&mut self) -> u8 {
        self.program_address += 1;
        self.program[self.program_address]
    }


    //Loop that runs until program ends or HALT upcode is reached
    pub fn run(&mut self, program: Vec<u8>, show: bool) -> u64 {
        self.program = program;
        self.show = show;

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

        if self.show {
            println!("{:?}, {}", opcode, self.program_address);
        }

        match opcode {
            //Ens program if a HALT or ILLEGAL upcode is found
            OpCode::Illegal => unimplemented!("program address: {}, {:?}", self.program_address, self.stack),
            OpCode::Halt => unimplemented!("program address: {}, {:?}", self.program_address, self.stack),

            /*
            Appends a number to the stack. 
            This number either has 8, 16, 32, or 64 bits depending on what is specified by the next 8 bits following the opcode
            */
            OpCode::Push => {
                let num = u64::from_be_bytes([self.next_8_bits(), self.next_8_bits(), self.next_8_bits(), self.next_8_bits(), self.next_8_bits(), self.next_8_bits(), self.next_8_bits(), self.next_8_bits()]);
                self.stack.push(num);
            },

            //Removes a number from the stack
            OpCode::Pop => {
                self.stack.pop();
            },

            //Mathematical operations performed on the last 2 numbers from the stack
            OpCode::AddU => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 + num1)
            },
            OpCode::SubU => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 - num1)
            },
            OpCode::MulU => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 * num1)
            },
            OpCode::DivU => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 / num1)
            },

            OpCode::AddI => {
                let [num1, num2] = self.double_pop();
                let result = i64::from_be_bytes(num2.to_be_bytes()) + i64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },
            OpCode::SubI => {
                let [num1, num2] = self.double_pop();
                let result = i64::from_be_bytes(num2.to_be_bytes()) - i64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },
            OpCode::MulI => {
                let [num1, num2] = self.double_pop();
                let result = i64::from_be_bytes(num2.to_be_bytes()) * i64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },
            OpCode::DivI => {
                let [num1, num2] = self.double_pop();
                let result = i64::from_be_bytes(num2.to_be_bytes()) / i64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },

            OpCode::AddF => {
                let [num1, num2] = self.double_pop();
                let result = f64::from_be_bytes(num2.to_be_bytes()) + f64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },
            OpCode::SubF => {
                let [num1, num2] = self.double_pop();
                let result = f64::from_be_bytes(num2.to_be_bytes()) - f64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },
            OpCode::MulF => {
                let [num1, num2] = self.double_pop();
                let result = f64::from_be_bytes(num2.to_be_bytes()) * f64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },
            OpCode::DivF => {
                let [num1, num2] = self.double_pop();
                let result = f64::from_be_bytes(num2.to_be_bytes()) / f64::from_be_bytes(num1.to_be_bytes());
                self.stack.push(u64::from_be_bytes(result.to_be_bytes()))
            },

            //Shifts th ebits in the number to the left or right depenidng on the number that immediateley follows the opcode
            OpCode::Shift => {
                let [num1, num2] = self.double_pop();
                if self.next_8_bits() == 0 {
                    self.stack.push(num2 << num1)
                }
                else {
                    self.stack.push(num2 >> num1)
                }
            },

            //Bitwise operations on the last 2 numbers in the stack
            OpCode::BitAnd => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 & num1);
            },
            OpCode::BitOr => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 | num1);
            },
            OpCode::BitXor => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 ^ num1);
            },
            OpCode::BitNot => {
                let num = match self.stack.pop() {
                    Some(x) => x,
                    None => unimplemented!()
                };
                self.stack.push(!num)
            }
        }
    }

    //Removes and returns the last 2 numbers form the stack
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