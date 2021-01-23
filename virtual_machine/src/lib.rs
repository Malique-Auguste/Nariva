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
            Opcode::Push.into(), 3, 255, 255, 255, 255, 255, 255, 255, 255,
            Opcode::Push.into(), 0, 1,
            Opcode::Pop.into()
        ]].concat();

        let mut vm = Machine::new(program);
        assert_eq!(u64::MAX, vm.run());
    }

    #[test]
    fn math_u() {
        let program: Vec<u8> = [HEADER.to_vec(), vec![
            Opcode::Push.into(), 0, 255,
            Opcode::Push.into(), 0, 255,
            Opcode::MulU.into(),

            Opcode::Push.into(), 0, 255,
            Opcode::DivU.into(),
            
            Opcode::Push.into(), 0, 255,
            Opcode::SubU.into(),
            
            Opcode::Push.into(), 0, 1,
            Opcode::AddU.into(),
        ]].concat();

        let mut machine = Machine::new(program);
        assert_eq!(1, machine.run())
    }

    #[test]
    fn math_i() {
        let program: Vec<u8> = [HEADER.to_vec(), vec![
            Opcode::Push.into(), 0, 255,
            Opcode::Push.into(), 0, 255,
            Opcode::MulI.into(),

            Opcode::Push.into(), 0, 255,
            Opcode::DivI.into(),
            
            Opcode::Push.into(), 0, 255,
            Opcode::SubI.into(),
            
            Opcode::Push.into(), 0, 1,
            Opcode::AddI.into(),
        ]].concat();

        let mut machine = Machine::new(program);
        assert_eq!(1, i64::from_be_bytes(machine.run().to_be_bytes()))
    }

    #[test]
    fn math_f() {
        let program: Vec<u8> = [HEADER.to_vec(), vec![
            Opcode::Push.into(), 3, 64, 111, 224, 0, 0, 0, 0, 0,        //255
            Opcode::Push.into(), 3, 64, 111, 224, 0, 0, 0, 0, 0,
            Opcode::MulF.into(),

            Opcode::Push.into(), 3, 64, 127, 224, 0, 0, 0, 0, 0,        //510
            Opcode::DivF.into(),
            
            Opcode::Push.into(), 3, 64, 95, 192, 0, 0, 0, 0, 0,         //127
            Opcode::SubF.into(),
            
            Opcode::Push.into(), 3, 63, 240, 0, 0, 0, 0, 0, 0,
            Opcode::AddF.into(),
        ]].concat();

        let mut machine = Machine::new(program);
        assert_eq!(1.5, f64::from_be_bytes(machine.run().to_be_bytes()))
    }
    
    #[test]
    fn bit_operations() {
        let program = [HEADER.to_vec(), vec![
            Opcode::Push.into(), 0, 15,
            Opcode::Push.into(), 0, 4,
            Opcode::Shift.into(), 0,

            Opcode::Push.into(), 0, 15,
            Opcode::BitAnd.into(),

            Opcode::Push.into(), 0, 10,
            Opcode::BitXor.into(),

            Opcode::Push.into(), 0, 5,
            Opcode::BitOr.into(),

            Opcode::BitNot.into(),
            Opcode::BitNot.into(),            
        ]].concat();

        let mut machine = Machine::new(program);
        assert_eq!(15, machine.run())
    }
    
}