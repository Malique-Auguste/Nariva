#[derive(Debug, PartialEq)]
pub enum Opcode {
    Illegal,
    Halt,

    Push,
    PushR,
    Pop,
    PopR,

    Add32,
    Sub32,
    Mul32,
    Div32,

    Add64,
    Sub64,
    Mul64,
    Div64,

    Shift,
    BitAnd,
    BitOr,
    BitXor,
    BitNot
}

pub fn encode(opcode: Opcode, operand1: u8, operand2: u8, operand3: u8) -> u32 {
    u32::from_be_bytes([opcode.into(), operand1, operand2, operand3])
}

pub fn decode(instruction: u32) -> (Opcode, u8, u8, u8) {
    let [opcode, operand1, operand2, operand3] = instruction.to_be_bytes();
    (opcode.into(), operand1, operand2, operand3)
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

            Opcode::Add32 => 6,
            Opcode::Sub32 => 7,
            Opcode::Mul32 => 8,
            Opcode::Div32 => 9,

            Opcode::Add64 => 10,
            Opcode::Sub64 => 11,
            Opcode::Mul64 => 12,
            Opcode::Div64 => 13,

            Opcode::Shift => 14, 
            
            Opcode::BitAnd => 15,
            Opcode::BitOr => 16,
            Opcode::BitXor => 17,
            Opcode::BitNot => 18,
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

            6 => Opcode::Add32,
            7 => Opcode::Sub32,
            8 => Opcode::Mul32,
            9 => Opcode::Div32,

            10 => Opcode::Add64,
            11 => Opcode::Sub64,
            12 => Opcode::Mul64,
            13 => Opcode::Div64,

            14 => Opcode::Shift,
            15 => Opcode::BitAnd,
            16 => Opcode::BitOr,
            17 => Opcode::BitXor,
            18 => Opcode::BitNot,
            
            _ => Opcode::Illegal,
        }
    }
}


pub trait ByteOps: std::ops::BitOr  + std::marker::Sized {
    type Smaller: std::ops::BitOr + std::ops::Shl  + std::marker::Sized ;

    fn split(&self) -> [Self; 2];
    fn split_smaller(&self) -> [Self::Smaller; 2];

    fn join(halves: (Self, Self)) -> <Self as std::ops::BitOr>::Output {
        halves.0 | halves.1
    }
    fn join_smaller(halves: [Self::Smaller; 2]) -> Self;
}

impl ByteOps for u16 {
    type Smaller = u8;

    fn split(&self) -> [Self; 2] {
        [self & 65280, self & 255]
    }

    fn split_smaller(&self) -> [Self::Smaller; 2] {
        [
            ((self & 65280) >> 8) as u8, 
            (self & 255) as u8
        ]
    }

    fn join_smaller(halves: [Self::Smaller; 2]) -> u16 {
        (halves[0] as u16) << 8 | halves[1] as u16
    }
}

impl ByteOps for u32 {
    type Smaller = u16;

    fn split(&self) -> [Self; 2] {
        [self & 4_294_901_760, self & 65535]
    }

    fn split_smaller(&self) -> [Self::Smaller; 2] {
        [
            ((self & 4_294_901_760) >> 16) as u16, 
            (self & 65535) as u16
        ]
    }

    fn join_smaller(halves: [Self::Smaller; 2]) -> Self {
        (halves[0] as u32) << 16 | halves[1] as u32
    }
}

impl ByteOps for u64 {
    type Smaller = u32;

    fn split(&self) -> [Self; 2] {
        [self & 18_446_744_069_414_584_320, self & 4_294_967_295]
    }

    fn split_smaller(&self) -> [Self::Smaller; 2] {
        [
            ((self & 18_446_744_069_414_584_320) >> 32) as u32, 
            (self & 4_294_967_295) as u32
        ]
    }

    fn join_smaller(halves: [Self::Smaller; 2]) -> Self {
        (halves[0] as u64) << 32 | halves[1] as u64
    }
}
