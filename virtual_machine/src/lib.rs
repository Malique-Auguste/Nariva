pub mod vm;
pub mod instruction;
pub mod flag;


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
            OpCode::Push.into(), 255, 255, 255, 255, 255, 255, 255, 255,
            OpCode::Push.into(), 0,0,0,0,0,0,0,1,
            OpCode::Pop.into()
        ]].concat();

        let mut vm = Machine::new();
        assert_eq!(u64::MAX, vm.run(program, false));
    }

    #[test]
    fn math_u() {
        let program: Vec<u8> = [HEADER.to_vec(), vec![
            OpCode::Push.into(), 0,0,0,0,0,0,0,55,
            OpCode::Push.into(), 0,0,0,0,0,0,0,55,
            OpCode::MulU.into(),

            OpCode::Push.into(), 0,0,0,0,0,0,0,55,
            OpCode::DivU.into(),
            
            OpCode::Push.into(), 0,0,0,0,0,0,0,55,
            OpCode::SubU.into(),
            
            OpCode::Push.into(), 0,0,0,0,0,0,0,1,
            OpCode::AddU.into(),
        ]].concat();

        let mut machine = Machine::new();
        assert_eq!(1, machine.run(program, false))
    }

    #[test]
    fn math_i() {
        let program: Vec<u8> = [HEADER.to_vec(), vec![
            OpCode::Push.into(), 0,0,0,0,0,0,0,255,
            OpCode::Push.into(), 0,0,0,0,0,0,0,255,
            OpCode::MulI.into(),

            OpCode::Push.into(), 0,0,0,0,0,0,0,255,
            OpCode::DivI.into(),
            
            OpCode::Push.into(), 0,0,0,0,0,0,0,255,
            OpCode::SubI.into(),
            
            OpCode::Push.into(), 0,0,0,0,0,0,0,1,
            OpCode::AddI.into(),
        ]].concat();

        let mut machine = Machine::new();
        assert_eq!(1, i64::from_be_bytes(machine.run(program, false).to_be_bytes()))
    }

    #[test]
    fn math_f() {
        let program: Vec<u8> = [HEADER.to_vec(), vec![
            OpCode::Push.into(), 64, 111, 224, 0, 0, 0, 0, 0,        //255
            OpCode::Push.into(), 64, 111, 224, 0, 0, 0, 0, 0,
            OpCode::MulF.into(),

            OpCode::Push.into(), 64, 127, 224, 0, 0, 0, 0, 0,        //510
            OpCode::DivF.into(),
            
            OpCode::Push.into(), 64, 95, 192, 0, 0, 0, 0, 0,         //127
            OpCode::SubF.into(),
            
            OpCode::Push.into(), 63, 240, 0, 0, 0, 0, 0, 0,
            OpCode::AddF.into(),
        ]].concat();

        let mut machine = Machine::new();
        assert_eq!(1.5, f64::from_be_bytes(machine.run(program, false).to_be_bytes()))
    }
    
    #[test]
    fn bit_operations() {
        let program = [HEADER.to_vec(), vec![
            OpCode::Push.into(), 0,0,0,0,0,0,0,15,
            OpCode::Push.into(), 0,0,0,0,0,0,0,4,
            OpCode::Shift.into(), 0,0,0,0,0,0,0,0,

            OpCode::Push.into(), 0,0,0,0,0,0,0,15,
            OpCode::BitAnd.into(),

            OpCode::Push.into(), 0,0,0,0,0,0,0,10,
            OpCode::BitXor.into(),

            OpCode::Push.into(), 0,0,0,0,0,0,0,5,
            OpCode::BitOr.into(),

            OpCode::BitNot.into(),
            OpCode::BitNot.into(),            
        ]].concat();

        let mut machine = Machine::new();
        assert_eq!(15, machine.run(program, false))
    }

    #[test]
    fn jumping() {
        let program = [HEADER.to_vec(), vec![
            OpCode::Push.into(), 0,0,0,0,0,0,0,15,
            OpCode::Push.into(), 0,0,0,0,0,0,0,10,
            OpCode::CMP.into(), 0,0,0,0,0,0,0,0,
            OpCode::JE.into(),
            OpCode::JL.into(),
            OpCode::JG.into(), 0,0,0,0,0,0,0,41,
            OpCode::Halt.into(),
            OpCode::Push.into(), 0,0,0,0,0,0,0,40, 
            OpCode::JMP.into()
            ]
        ].concat();

        let mut machine = Machine::new();
        assert_eq!(u64::MAX, machine.run(program, true))
    }

    #[test]
    fn parse_function() {
        let program = [HEADER.to_vec(), vec![
            OpCode::Push.into(), 0,0,0,0,0,0,0,12,
            OpCode::Push.into(), 0,0,0,0,0,0,0,17,
            OpCode::CMP.into(), 0,0,0,0,0,0,0,0,
            OpCode::JG.into(), 0,0,0,0,0,0,0,61,
            OpCode::Call.into(), 0,0,0,0,0,0,0,62,
            OpCode::Halt.into(),


            OpCode::Push.into(), 0,0,0,0,0,0,0,40, 
            OpCode::Return.into(),
            
            ]
        ].concat();

        let mut machine = Machine::new();
        assert_eq!(40, machine.run(program, true))
    }

    #[test]
    fn printing() {
        let program = [HEADER.to_vec(), vec![
            OpCode::Push.into(), 0,0,0,0,0,0,0,10,
            OpCode::Push.into(), 0,0,0,0,0,0,0,97,
            OpCode::Print.into(), 0,0,0,0,0,0,0,3
        ]].concat();

        let mut machine = Machine::new();
        assert_eq!(10, machine.run(program, false))
    }

    fn neg_jump() {
        let program = [HEADER.to_vec(), vec![
            OpCode::Push.into(), 0,0,0,0,0,0,0,10,
            OpCode::Push.into(), 0,0,0,0,0,0,0,5,
            OpCode::CMP.into(),
            OpCode::JNE.into(), 2,
            OpCode::Halt.into(),
            OpCode::Push.into(), 0,0,0,0,0,0,0,5,
            OpCode::Dupli.into(),
            OpCode::CMP.into(),
            OpCode::JE.into(), 255, 255, 255, 255, 255, 255, 255, 252,
            OpCode::Print.into(), 0,0,0,0,0,0,0,3
        ]].concat();
    }
    
}