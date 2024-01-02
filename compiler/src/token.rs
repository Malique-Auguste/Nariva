use virtual_machine::instruction::OpCode;

#[derive(Debug, PartialEq)]
pub enum Token {
    OpCode (String),
    Func (String),
    NumU (u64),
    NumI (i64),
    NumF (f64)
}

impl Token {
    pub fn is_push(&self) -> bool {
        match self {
            Token::OpCode(s) => {
                if s == "PUSH" || s == "Push" {
                    true
                }
                else {
                    false
                }
            },
            _ => false
        }
    }

    pub fn is_conditional_jmp(&self) -> bool {
        match self {
            Token::OpCode(s) => {
                if s == "JE" || s == "JNE" || s == "JG" || s == "JL" {
                    true
                }
                else {
                    false
                }
            },
            _ => false
        }
    }

    pub fn is_num(&self) -> bool {
        match self {
            Token::NumU(_) | Token::NumI(_) | Token::NumF(_) => true,
            _ => false
        }
    }

}