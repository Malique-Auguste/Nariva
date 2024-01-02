extern crate virtual_machine;

use virtual_machine::vm::Machine;
use std::fs;

fn main() {
    let mut vm = Machine::new();
    let file_data = fs::read("nar files/fizzbuzz.nar").unwrap();
    let be_bytes = vm.run(file_data, true).to_be_bytes();
    println!("{}",  f64::from_be_bytes(be_bytes));
}
