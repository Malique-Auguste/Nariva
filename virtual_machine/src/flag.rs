//When the vm is comparing two numbers it will use a flag as the result
#[derive(Debug)]
pub enum Flag {
    None,
    Equal,
    Less,
    Greater,
    Overflow,
}