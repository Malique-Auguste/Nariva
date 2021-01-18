#[derive(Debug, PartialEq)]
pub enum Opcode {
    Illegal,
    Halt,

    Push,
    PushR,
    Pop,
    PopR,
}

pub fn encode(opcode: Opcode, operand1: u16, operand2: u8) -> u32 {
    (opcode as u32) << 24 | (operand1 as u32) << 8 | (operand2 as u32) 
}

pub fn decode(instruction: u32) -> (Opcode, u16, u8) {
    let [opcode, operand1_upper, operand1_lower, operand2] = instruction.to_be_bytes();

    (
        opcode.into(),
        ((operand1_upper as u16) << 8 | operand1_lower as u16),
        operand2 as u8,
    )
}

pub fn merge_u16(upper_8_bits: u8, lower_8_bits: u8) -> u16 {
    (upper_8_bits as u16) << 8 | lower_8_bits as u16
}

pub fn separate_u16(num: u16) -> (u8, u8) {
    (upper_8_bits(num), lower_8_bits(num))
}

pub fn upper_8_bits(num: u16) -> u8 {
    ((num & 65280) >> 8) as u8
}

pub fn lower_8_bits(num: u16) -> u8 {
    (num & 255) as u8
}

impl From<Opcode> for u8 {
    fn from(o: Opcode) -> u8 {
        match o {
            Opcode::Illegal => 0,
            Opcode::Halt => 1,

            Opcode::Push => 2,
            Opcode::PushR => 3,
            Opcode::Pop => 4,
            Opcode::PopR => 5,
        }
    }
}

impl From<u8> for Opcode {
    fn from(o: u8) -> Opcode {
        match o {
            1 => Opcode::Halt,

            2 => Opcode::Push,
            3 => Opcode::PushR,
            4 => Opcode::Pop,
            5 => Opcode::PopR,
            
            _ => Opcode::Illegal,
        }
    }
}