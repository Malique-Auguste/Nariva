pub mod vm;
pub mod instruction;

#[cfg(test)]
mod vm_tests {
    use crate::instruction::*;
    use crate::vm::*;

    #[test]
    fn split_trait() {
        let num:u16 = 5_123;
        assert_eq!(num, u16::join((num.split()[0], num.split()[1])));

        let num:u32 = 12_556;
        assert_eq!(num, u32::join((num.split()[0], num.split()[1])));

        let num:u64 = 77_326_883_805_641_601;
        assert_eq!(num, u64::join((num.split()[0], num.split()[1])));
    }

    #[test]
    fn push_pop() {
        let program: Vec<u8> = [HEADER.to_vec(), vec![
            Opcode::Push.into(), 0, 0, 100,
            Opcode::Push.into(), 0, 0, 50,
            Opcode::Pop.into(), 1, 0,
            Opcode::Push.into(), 1, 0,
            Opcode::Pop.into(), 0,
        ]].concat();

        let mut vm = Machine::new(program);
        assert_eq!(100, vm.run());
    }

    /*
    #[test]
    fn bit_operations() {
        let program = vec![
            Opcode::Push.into(), 0, 0, 15,
            Opcode::Pop.into(), 1, 0,

        ];
        let program = vec![
            encode(Opcode::Push, 0, 15, 0),
            encode(Opcode::PopR, 0, 0, 0),
            encode(Opcode::Push, 0, 2, 0),
            encode(Opcode::PopR, 1, 0, 0),
            encode(Opcode::Shift, 0, 1, 0),
            encode(Opcode::PushR, 0, 0, 0),
        ];

        let mut vm = Machine::new(program);
        assert_eq!(60, vm.run());

        let program = vec![
            encode(Opcode::Push, 0, 240, 0),
            encode(Opcode::PopR, 0, 0, 0),
            encode(Opcode::Push, 0, 15, 0),
            encode(Opcode::PopR, 1, 0, 0),
            encode(Opcode::BitAnd, 0, 1, 0),
            encode(Opcode::PushR, 0, 0, 0),
        ];

        let mut vm = Machine::new(program);
        assert_eq!(0, vm.run());

        let program = vec![
            encode(Opcode::Push, 0, 240, 0),
            encode(Opcode::PopR, 0, 0, 0),
            encode(Opcode::Push, 0, 15, 0),
            encode(Opcode::PopR, 1, 0, 0),
            encode(Opcode::BitOr, 0, 1, 0),
            encode(Opcode::PushR, 0, 0, 0),
        ];

        let mut vm = Machine::new(program);
        assert_eq!(255, vm.run());

        let program = vec![
            encode(Opcode::Push, 0, 246, 0),
            encode(Opcode::PopR, 0, 0, 0),
            encode(Opcode::Push, 0, 111, 0),
            encode(Opcode::PopR, 1, 0, 0),
            encode(Opcode::BitXor, 0, 1, 0),
            encode(Opcode::PushR, 0, 0, 0),
        ];

        let mut vm = Machine::new(program);
        assert_eq!(153, vm.run());

        let program = vec![
            encode(Opcode::Push, 0, 191, 0),
            encode(Opcode::Push, 0, 189, 0),
            encode(Opcode::Push, 0, 240, 0),
            encode(Opcode::Push, 0, 255, 0),

            encode(Opcode::PopR, 0, 0, 0),
            encode(Opcode::Shift, 0, 8, 1),
            
            encode(Opcode::PopR, 1, 0, 0),
            encode(Opcode::BitOr, 0, 1, 0),
            encode(Opcode::Shift, 0, 8, 1),
            
            encode(Opcode::PopR, 1, 0, 0),
            encode(Opcode::BitOr, 0, 1, 0),
            encode(Opcode::Shift, 0, 8, 1),
            
            encode(Opcode::PopR, 1, 0, 0),
            encode(Opcode::BitOr, 0, 1, 0),

            encode(Opcode::BitNot, 0, 0, 0),
            encode(Opcode::PushR, 0, 0, 0),
            
        ];

        let mut vm = Machine::new(program);
        assert_eq!(1_000_000, vm.run());
    }
    */
}