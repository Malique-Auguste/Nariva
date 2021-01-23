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

    #[test]
    fn math_u() {
        let program: Vec<u8> = [HEADER.to_vec(), vec![
            Opcode::Push.into(), 0, 0, 255,
            Opcode::Push.into(), 0, 0, 255,
            Opcode::MulU.into(),

            Opcode::Push.into(), 0, 0, 255,
            Opcode::DivU.into(),
            
            Opcode::Push.into(), 0, 0, 255,
            Opcode::SubU.into(),
            
            Opcode::Push.into(), 0, 0, 1,
            Opcode::AddU.into(),
        ]].concat();

        let mut machine = Machine::new(program);
        assert_eq!(1, machine.run())
    }

    #[test]
    fn math_i() {
        let program: Vec<u8> = [HEADER.to_vec(), vec![
            Opcode::Push.into(), 0, 0, 255,
            Opcode::Push.into(), 0, 0, 255,
            Opcode::MulI.into(),

            Opcode::Push.into(), 0, 0, 255,
            Opcode::DivI.into(),
            
            Opcode::Push.into(), 0, 0, 255,
            Opcode::SubI.into(),
            
            Opcode::Push.into(), 0, 0, 1,
            Opcode::AddI.into(),
        ]].concat();

        let mut machine = Machine::new(program);
        assert_eq!(1, i64::from_be_bytes(machine.run().to_be_bytes()))
    }

    #[test]
    fn math_f() {
        let program: Vec<u8> = [HEADER.to_vec(), vec![
            Opcode::Push.into(), 0, 0, 255,
            Opcode::Push.into(), 0, 0, 255,
            Opcode::MulF.into(),

            Opcode::Push.into(), 0, 0, 255,
            Opcode::DivF.into(),
            
            Opcode::Push.into(), 0, 0, 255,
            Opcode::SubF.into(),
            
            Opcode::Push.into(), 0, 0, 1,
            Opcode::AddF.into(),
        ]].concat();

        let mut machine = Machine::new(program);
        assert_eq!(1.0, f64::from_be_bytes(machine.run().to_be_bytes()))
    }
    
    #[test]
    fn bit_operations() {
        let program = [HEADER.to_vec(), vec![
            Opcode::Push.into(), 0, 0, 240,
            Opcode::Pop.into(), 1, 0,
            Opcode::Shift.into(), 3, 0, 4,

            Opcode::Push.into(), 0, 0, 82,
            Opcode::Pop.into(), 1, 1,
            Opcode::BitAnd.into(), 0, 1,
            
            Opcode::Push.into(), 0, 0, 167,
            Opcode::Pop.into(), 1, 1,
            Opcode::BitAnd.into(), 0, 1,
            
            Opcode::BitNot.into(), 0,
            Opcode::BitNot.into(), 0,
            
            Opcode::Push.into(), 0, 0, 170,
            Opcode::Pop.into(), 1, 1,
            Opcode::BitXor.into(), 0, 1,

            Opcode::Push.into(), 1, 0
            
        ]].concat();

        let mut machine = Machine::new(program);
        assert_eq!(168, machine.run())
    }
    
}