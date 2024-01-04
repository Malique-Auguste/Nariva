use crate::instruction::{OpCode, ByteOps};
use crate::flag::Flag;

//Code at the start of all programs to ensure that they are nariva files.
//The numebrs decode to "Nariva Executable"
pub const HEADER: [u8; 17] = [78, 97, 114, 105, 118, 97, 32, 69, 120, 101, 99, 117, 116, 97, 98, 108, 101];

pub struct Machine {
    //list of encoded instructions
    pub program: Vec<u8>,
    
    //current position of the vm in the list of instructions
    pub program_address: usize,

    //numbers stored by the vm / working memory / ram
    pub stack: Vec<u64>,
    pub return_addresses: Vec<usize>,
    pub registers: [u64; 4],

    //whether or not to print out instructions being executed
    show: bool,

    //result of comparison
    flag: Flag
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            program: Vec::new(),
            program_address: 0,
            stack: Vec::new(),
            return_addresses: Vec::new(),
            registers: [0,0,0,0],
            show: false,
            flag: Flag::None
        }
    }

    pub fn next_8_bits(&mut self) -> u8 {
        self.program_address += 1;
        self.program[self.program_address]
    }

    pub fn next_64_bits(&mut self) -> u64 {
        self.program_address += 8;
        u64::from_be_bytes([self.program[self.program_address - 7 ], self.program[self.program_address - 6 ],
            self.program[self.program_address - 5 ], self.program[self.program_address - 4 ],
            self.program[self.program_address - 3 ], self.program[self.program_address - 2 ],
            self.program[self.program_address - 1 ], self.program[self.program_address ]])
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
                    return match self.stack.pop() {
                        Some(num) => num,
                        None => u64::MAX
                    }
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
            println!("{:?}, {}, {}, {:?}, {:?}", opcode, self.program_address, self.program_address - HEADER.len(), self.stack, self.registers);
        }

        match opcode {
            //Ends program if a HALT or ILLEGAL upcode is found
            OpCode::Illegal => {
                println!("\nIllegal / unknown upcode reached at {}, current stack: {:?}", self.program_address, self.stack);
                println!("Program: {:?}\nReturn Address: {:?}", self.program, self.return_addresses);
                println!("Flag: {:?}", self.flag);
                panic!()
            },

            OpCode::Halt => self.program_address = self.program.len(),

            /*
            Appends a number to the stack. 
            This number either has 8, 16, 32, or 64 bits depending on what is specified by the next 8 bits following the opcode
            */
            OpCode::Push => {
                let new_num = self.next_64_bits();
                self.stack.push(new_num)
            },

            //Removes a number from the stack
            OpCode::Pop => {
                self.stack.pop();
            },

            //Mathematical operations performed on the last 2 numbers from the stack
            OpCode::AddU => {
                let [num1, num2] = self.double_pop();
                match num2.checked_add(num1) {
                    Some(result) => self.stack.push(result),
                    None => {
                        self.stack.push(u64::MAX);
                        self.flag = Flag::Overflow;
                    }

                };
            },
            OpCode::SubU => {
                let [num1, num2] = self.double_pop();
                match num2.checked_sub(num1) {
                    Some(result) => self.stack.push(result),
                    None => {
                        self.stack.push(u64::MIN);
                        self.flag = Flag::Overflow;
                    }

                };
            },
            OpCode::MulU => {
                let [num2, num1] = self.double_pop();
                match num1.checked_mul(num2) {
                    Some(result) => self.stack.push(result),
                    None => {
                        self.stack.push(u64::MAX);
                        self.flag = Flag::Overflow;
                    }

                };
            },
            OpCode::DivU => {
                let [num1, num2] = self.double_pop();
                match num2.checked_div(num1) {
                    Some(result) => self.stack.push(result),
                    None => {
                        self.stack.push(u64::MAX);
                        self.flag = Flag::Overflow;
                    }

                };
            },

            OpCode::AddI => {
                let [num1, num2] = self.double_pop();
                let [num1, num2] = [i64::from_be_bytes(num1.to_be_bytes()), i64::from_be_bytes(num2.to_be_bytes())];
                match num2.checked_add(num1) {
                    Some(result) => self.stack.push(u64::from_be_bytes(result.to_be_bytes())),
                    None => {
                        self.stack.push(u64::MAX);
                        self.flag = Flag::Overflow;
                    }

                };
            },
            OpCode::SubI => {
                let [num1, num2] = self.double_pop();
                let [num1, num2] = [i64::from_be_bytes(num1.to_be_bytes()), i64::from_be_bytes(num2.to_be_bytes())];
                match num2.checked_sub(num1) {
                    Some(result) => self.stack.push(u64::from_be_bytes(result.to_be_bytes())),
                    None => {
                        self.stack.push(u64::MIN);
                        self.flag = Flag::Overflow;
                    }

                };
            },
            OpCode::MulI => {
                let [num1, num2] = self.double_pop();
                let [num1, num2] = [i64::from_be_bytes(num1.to_be_bytes()), i64::from_be_bytes(num2.to_be_bytes())];
                match num2.checked_mul(num1) {
                    Some(result) => self.stack.push(u64::from_be_bytes(result.to_be_bytes())),
                    None => {
                        self.stack.push(u64::MAX);
                        self.flag = Flag::Overflow;
                    }

                };
            },
            OpCode::DivI => {
                let [num1, num2] = self.double_pop();
                let [num1, num2] = [i64::from_be_bytes(num1.to_be_bytes()), i64::from_be_bytes(num2.to_be_bytes())];
                match num2.checked_div(num1) {
                    Some(result) => self.stack.push(u64::from_be_bytes(result.to_be_bytes())),
                    None => {
                        self.stack.push(u64::MAX);
                        self.flag = Flag::Overflow;
                    }

                };
            },

            OpCode::AddF => {
                let [num1, num2] = self.double_pop();
                let result = f64::from_be_bytes(num1.to_be_bytes()) + f64::from_be_bytes(num2.to_be_bytes());
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
                if self.next_64_bits() == 0 {
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
            },

            OpCode::CMP => {
                match self.next_64_bits() {
                    0 => {
                        let [num1, num2] = self.double_pop();
                        self.flag =  match num2.checked_sub(num1) {
                            Some(0) => Flag::Equal,
                            Some(_) => Flag::Greater,
                            None => Flag::Less
                        }
                    },
                    1 => {
                        let [num1, num2] = self.double_pop();
                        self.flag = match i64::from_be_bytes(num2.to_be_bytes()).checked_sub(i64::from_be_bytes(num1.to_be_bytes())) {
                            Some(0) => Flag::Equal,
                            Some(n) => {
                                if n > 0 {
                                    Flag::Greater
                                } 
                                else {
                                    Flag::Less
                                }
                            },
                            None => Flag::Less
                        }

                    },
                    2 => {
                        let [num1, num2] = self.double_pop();
                        let result: f64 = f64::from_be_bytes(num2.to_be_bytes()) - f64::from_be_bytes(num1.to_be_bytes());
                        if result > 0.0 {
                            self.flag = Flag::Greater;
                        }
                        else if result == 0.0 {
                            self.flag = Flag::Equal;
                        }
                        else {
                            self.flag = Flag::Less;
                        }
                    },
                    _ => unimplemented!("") 
                }
            },

            OpCode::JMP => unimplemented!(),

            OpCode::JE => {
                match self.flag {
                    Flag::Equal => {
                        self.jump()
                    },
                    _ => self.program_address += 8
                } 
            },

            OpCode::JNE => {
                match self.flag {
                    Flag::Greater | Flag::Less => {
                        self.jump()
                    },
                    _ => self.program_address += 8
                } 
            },

            OpCode::JG => {
                match self.flag {
                    Flag::Greater => {
                        self.jump()
                    },
                    _ => self.program_address += 8
                } 
            },

            OpCode::JL => {
                match self.flag {
                    Flag::Less => {
                        self.jump()
                    },
                    _ => self.program_address += 8
                } 
            },

            OpCode::Call => {
                self.return_addresses.push(self.program_address + 8);
                self.program_address = self.next_64_bits() as usize;
            },

            OpCode::Return => {
                self.program_address = self.return_addresses.pop().unwrap()
            },

            OpCode::ModU => {
                let [num1, num2] = self.double_pop();
                self.stack.push(num2 % num1)
            },

            OpCode::ModI => {
                let [num1, num2] = self.double_pop();
                let [num1, num2] = [i64::from_be_bytes(num1.to_be_bytes()), i64::from_be_bytes(num2.to_be_bytes())];
                self.stack.push(u64::from_be_bytes((num2 % num1).to_be_bytes()))
            },

            OpCode::ModF => {
                let [num1, num2] = self.double_pop();
                let [num1, num2] = [f64::from_be_bytes(num1.to_be_bytes()), f64::from_be_bytes(num2.to_be_bytes())];
                self.stack.push(u64::from_be_bytes((num2 % num1).to_be_bytes()))
            },

            OpCode::Print => {
                match self.next_64_bits() {
                    0 => println!("{}", self.stack.pop().unwrap() as u64),
                    1 => println!("{}", self.stack.pop().unwrap() as i64),
                    2 => println!("{}", self.stack.pop().unwrap() as f64),
                    3 => println!("{}", self.stack.pop().unwrap() as u8 as char),
                    _ => unimplemented!()

                }
            },

            OpCode::Dupli => {
                let num = self.stack.pop().unwrap();
                self.stack.push(num);
                self.stack.push(num)
            },

            OpCode::Store => {
                let register_index = self.next_64_bits() as usize;
                self.registers[register_index] = self.stack.pop().unwrap()
            }

            OpCode::Load => {
                let register_index = self.next_64_bits() as usize;
                self.stack.push(self.registers[register_index]);
            }
        }
    }

    fn jump(&mut self) {
        let jump_bits = self.next_64_bits();
        let int_jump_dist = i64::from_be_bytes(jump_bits.to_be_bytes());

        //Minus 8 bits because next_64_bits() already increases address by 8 and next iteration goes to opcode after given address
        self.program_address -= 9;

        if int_jump_dist > 0 {
            self.program_address += int_jump_dist.abs() as usize
        }
        else {
            self.program_address -= int_jump_dist.abs() as usize
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