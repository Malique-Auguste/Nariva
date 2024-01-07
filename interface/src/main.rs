extern crate virtual_machine;
extern crate compiler;


use virtual_machine::vm::Machine;
use compiler::compiler::Compiler;
use std::fs;
use std::io;

fn main() {
    /*
    let mut vm = Machine::new();
    let file_data = fs::read("nar files/fizzbuzz.nar").unwrap();
    let be_bytes = vm.run(file_data, false).to_be_bytes();
    println!("{}",  u64::from_be_bytes(be_bytes));
    */

    //https://patorjk.com/software/taag/#p=display&f=ANSI%20Shadow&t=Nariva 
    //ansi shadow font
    println!("
    ███╗   ██╗ █████╗ ██████╗ ██╗██╗   ██╗ █████╗ 
    ████╗  ██║██╔══██╗██╔══██╗██║██║   ██║██╔══██╗
    ██╔██╗ ██║███████║██████╔╝██║██║   ██║███████║
    ██║╚██╗██║██╔══██║██╔══██╗██║╚██╗ ██╔╝██╔══██║
    ██║ ╚████║██║  ██║██║  ██║██║ ╚████╔╝ ██║  ██║
    ╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝  ╚═╝  ╚═╝
    ----------------------------------------------                                           
    ");

    loop {
        println!("
        What would you like to do?
            1) Compile a .nar file
            2) Run a .binar file
            3) Compile and run a .nar file
            4) Exit program
        ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u64 = input.trim().parse().unwrap();

        match input {
            1 => {
                let (file_name, file_data) = match get_file_data() {
                    Ok(data) => data,
                    Err(e) => {
                        println!("Error in reading file: {}\nReloading...", e);
                        continue;
                    }
                };

                let mut comp = Compiler::new(format!("nar files/{}.bin", file_name));
                match comp.compile(file_data, false) {
                    Ok(_) => (),
                    Err(e) => println!("Error in compiling: {:?}.\nReloading...", e)
                }
                

            }
        }
    }
}

fn get_file_data() -> Result<(String, Vec<u8>), String> {
    println!("\nEnter the file name:");

    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read line");

        file_name.push_str(".nar");

    match fs::read(format!("nar files/{}", file_name.trim())) {
        Ok(data) => Ok((file_name, data)),
        Err(e) => Err(e.to_string())
    }
}
