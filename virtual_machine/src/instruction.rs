#[derive(Debug, PartialEq)]
pub enum Opcode {
    /*
    This represents an unknown opcode. 
    For example, the opcodes go up to 20, so any number after this gets defulted to an illegal upcode (0).
    */
    Illegal,

    //This represents the end fo the program
    Halt,

    //Adds a umber to the stack
    Push,
    //Removes a number from the stack
    Pop,

    //Math operations to be perfromed on unsigned intergers (>0)
    AddU,
    SubU,
    MulU,
    DivU,

    //Math operations to be performed on signes intergers (positive and negative)
    AddI,
    SubI,
    MulI,
    DivI,

    //Math operations to be performed on floats (fractions)
    AddF,
    SubF,
    MulF,
    DivF,

    //Moves the bits in a number the specified amount to the left
    Shift,

    //Bitwise operations
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

            Opcode::AddU => 4,
            Opcode::SubU => 5,
            Opcode::MulU => 6,
            Opcode::DivU => 7,

            Opcode::AddI => 8,
            Opcode::SubI => 9,
            Opcode::MulI => 10,
            Opcode::DivI => 11,

            Opcode::AddF => 12,
            Opcode::SubF => 13,
            Opcode::MulF => 14,
            Opcode::DivF => 15,

            Opcode::Shift => 16, 
            
            Opcode::BitAnd => 17,
            Opcode::BitOr => 18,
            Opcode::BitXor => 19,
            Opcode::BitNot => 20,
        }
    }
}

impl From<u8> for Opcode {
    fn from(o: u8) -> Opcode {
        match o {
            1 => Opcode::Halt,

            2 => Opcode::Push,
            3 => Opcode::Pop,

            4 => Opcode::AddU,
            5 => Opcode::SubU,
            6 => Opcode::MulU,
            7 => Opcode::DivU,

            8 => Opcode::AddI,
            9 => Opcode::SubI,
            10 => Opcode::MulI,
            11 => Opcode::DivI,

            12 => Opcode::AddF,
            13 => Opcode::SubF,
            14 => Opcode::MulF,
            15 => Opcode::DivF,

            16 => Opcode::Shift,
            17 => Opcode::BitAnd,
            18 => Opcode::BitOr,
            19 => Opcode::BitXor,
            20 => Opcode::BitNot,
            
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
