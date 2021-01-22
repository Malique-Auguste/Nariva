#[derive(Debug, PartialEq)]
pub enum Opcode {
    Illegal,
    Halt,

    Push,
    Pop,

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

impl From<Opcode> for u8 {
    fn from(o: Opcode) -> u8 {
        match o {
            Opcode::Illegal => 0,
            Opcode::Halt => 1,

            Opcode::Push => 2,
            Opcode::Pop => 3,

            Opcode::Add32 => 4,
            Opcode::Sub32 => 5,
            Opcode::Mul32 => 6,
            Opcode::Div32 => 7,

            Opcode::Add64 => 8,
            Opcode::Sub64 => 9,
            Opcode::Mul64 => 10,
            Opcode::Div64 => 11,

            Opcode::Shift => 12, 
            
            Opcode::BitAnd => 13,
            Opcode::BitOr => 14,
            Opcode::BitXor => 15,
            Opcode::BitNot => 16,
        }
    }
}

impl From<u8> for Opcode {
    fn from(o: u8) -> Opcode {
        match o {
            1 => Opcode::Halt,

            2 => Opcode::Push,
            3 => Opcode::Pop,

            4 => Opcode::Add32,
            5 => Opcode::Sub32,
            6 => Opcode::Mul32,
            7 => Opcode::Div32,

            8 => Opcode::Add64,
            9 => Opcode::Sub64,
            10 => Opcode::Mul64,
            11 => Opcode::Div64,

            12 => Opcode::Shift,
            13 => Opcode::BitAnd,
            14 => Opcode::BitOr,
            15 => Opcode::BitXor,
            16 => Opcode::BitNot,
            
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
