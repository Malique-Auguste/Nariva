pub mod vm;
pub mod instruction;

#[cfg(test)]
mod vm_tests {
    use crate::instruction::*;

    #[test]
    fn instruction_conversion() {
        assert_eq!(2, u8::from(Opcode::from(2)));
        assert_eq!(Opcode::Illegal, 0.into());

        let instruction = encode(Opcode::Pop, 12, 108);
        assert_eq!((Opcode::Pop, 12, 108), decode(instruction));

        assert_eq!((209, 34), separate_u16(merge_u16(209, 34)))
    }
}