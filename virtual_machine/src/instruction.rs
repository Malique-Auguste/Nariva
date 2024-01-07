//these are the instructions for the vm

#[derive(Debug, PartialEq)]
pub enum OpCode {
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

    //Moves the bits in a number the specified amount to the left or right
    Shift,

    //Bitwise operations
    BitAnd,
    BitOr,
    BitXor,
    BitNot,

    //Compare
    CMP,

    //jump if equal, not equal, greater, less
    JMP,
    JE,
    JNE,
    JG,
    JL,

    Call,
    Return,

    ModU,
    ModI,
    ModF,
    Print,

    Dupli,
    Store,
    Load,

    PrintSTR,
}

impl From<OpCode> for u8 {
    fn from(o: OpCode) -> u8 {
        match o {
            OpCode::Illegal => 0,
            OpCode::Halt => 1,

            OpCode::Push => 2,
            OpCode::Pop => 3,

            OpCode::AddU => 4,
            OpCode::SubU => 5,
            OpCode::MulU => 6,
            OpCode::DivU => 7,

            OpCode::AddI => 8,
            OpCode::SubI => 9,
            OpCode::MulI => 10,
            OpCode::DivI => 11,

            OpCode::AddF => 12,
            OpCode::SubF => 13,
            OpCode::MulF => 14,
            OpCode::DivF => 15,

            OpCode::Shift => 16, 
            
            OpCode::BitAnd => 17,
            OpCode::BitOr => 18,
            OpCode::BitXor => 19,
            OpCode::BitNot => 20,

            OpCode::CMP => 21,

            OpCode::JMP => 22,
            OpCode::JE => 23,
            OpCode::JNE => 24,
            OpCode::JG => 25,
            OpCode::JL => 26,

            OpCode::Call => 27,
            OpCode::Return => 28,

            OpCode::ModU => 29,
            OpCode::ModI => 30,
            OpCode::ModF => 31,
            OpCode::Print => 32,

            //Duplicate
            OpCode::Dupli => 33,

            OpCode::Store => 34,
            OpCode::Load => 35,

            OpCode::PrintSTR => 36
        }
    }
}

impl From<u8> for OpCode {
    fn from(o: u8) -> OpCode {
        match o {
            1 => OpCode::Halt,

            2 => OpCode::Push,
            3 => OpCode::Pop,

            4 => OpCode::AddU,
            5 => OpCode::SubU,
            6 => OpCode::MulU,
            7 => OpCode::DivU,

            8 => OpCode::AddI,
            9 => OpCode::SubI,
            10 => OpCode::MulI,
            11 => OpCode::DivI,

            12 => OpCode::AddF,
            13 => OpCode::SubF,
            14 => OpCode::MulF,
            15 => OpCode::DivF,

            16 => OpCode::Shift,
            17 => OpCode::BitAnd,
            18 => OpCode::BitOr,
            19 => OpCode::BitXor,
            20 => OpCode::BitNot,

            21 => OpCode::CMP,

            22 => OpCode::JMP,
            23 => OpCode::JE,
            24 => OpCode::JNE,
            25 => OpCode::JG,
            26 => OpCode::JL,

            27 => OpCode::Call,
            28 => OpCode::Return,

            29 => OpCode::ModU,
            30 => OpCode::ModI,
            31 => OpCode::ModF,

            32 => OpCode::Print,

            33 => OpCode::Dupli,

            34 => OpCode::Store,
            35 => OpCode::Load,
            36 => OpCode::PrintSTR,
            
            _ => OpCode::Illegal,
        }
    }
}

impl From<&String> for OpCode {
    fn from(o: &String) -> OpCode {
        match o.as_str() {
            "Halt" | "HALT" => OpCode::Halt,
            
            "Push" | "PUSH"=> OpCode::Push,
            "Pop" | "POP"=> OpCode::Pop,
            
            "AddU" | "ADDU" => OpCode::AddU,
            "SubU" | "SUBU" => OpCode::SubU,
            "MulU" | "MULU" => OpCode::MulU,
            "DivU" | "DIVU" => OpCode::DivU,

            "AddI" | "ADDI" => OpCode::AddI,
            "SubI" | "SUBI" => OpCode::SubI,
            "MulI" | "MULI" => OpCode::MulI,
            "DivI" | "DIVI" => OpCode::DivI,

            "AddF" | "ADDF" => OpCode::AddF,
            "SubF" | "SUBF" => OpCode::SubF,
            "MulF" | "MULF" => OpCode::MulF,
            "DivF" | "DIVF" => OpCode::DivF,

            "Shift" | "SHIFT" => OpCode::Shift,

            "BitAnd" | "BITAND" => OpCode::BitAnd,
            "BitOr" | "BITOR" => OpCode::BitOr,
            "BitXor" | "BITXOR" => OpCode::BitXor,
            "BitNot" | "BITNOT" => OpCode::BitNot,

            "Cmp" | "CMP" => OpCode::CMP,

            "JMP" => OpCode::JMP,
            "JE" => OpCode::JE,
            "JNE" => OpCode::JNE,
            "JG" => OpCode::JG,
            "JL" => OpCode::JL,

            "Call" | "CALL" => OpCode::Call,
            "Return" | "RETURN" => OpCode::Return,

            "ModU" | "MODU" => OpCode::ModU,
            "ModI" | "MODI" => OpCode::ModI,
            "ModF" | "MODF" => OpCode::ModF,

            "Print" | "PRINT" => OpCode::Print,
            "Dupli" | "DUPLI" => OpCode::Dupli,

            "Store" | "STORE" => OpCode::Store,
            "Load" | "LOAD" => OpCode::Load,

            "PrintSTR" | "PRINTSTR" => OpCode::PrintSTR,

            "Illegal" | "ILLEGAL" | _ => OpCode::Illegal,
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
