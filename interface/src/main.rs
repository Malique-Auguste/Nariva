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
----------------------------------------------");

    println!("\nRun in debug mode (y/n)?");
    let mut debug_mode = String::new();
        io::stdin()
            .read_line(&mut debug_mode)
            .expect("Failed to read line");

    let debug_mode: bool = match debug_mode.trim().parse().unwrap() {
        'y' | 'Y' => {
            println!("Debug mode activated...");
            true
        },
        _ => {
            println!("Debug mode deactivated...");
            false
        },
    };



    loop {
        println!("
What would you like to do?
    1) Compile a .nar file
    2) Run a .binar file
    3) Compile and run a .nar file
    4) Exit program");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u64 = input.trim().parse().unwrap();

        match input {
            1 => {
                let (file_name, file_data) = match get_file_data(false) {
                    Ok(data) => data,
                    Err(e) => {
                        println!("Error in reading file: {}\nReloading...", e);
                        continue;
                    }
                };

                let mut comp = Compiler::new(format!("nar files/{}.binar", file_name));
                match comp.compile(String::from_utf8_lossy(&file_data), debug_mode) {
                    Ok(_) => println!("\nSuccessfuly compiled: {}.nar", file_name),
                    Err(e) => println!("Error in compiling: {:?}.\nReloading...", e)
                }
            },

            2 => {
                let (file_name, file_data) = match get_file_data(true) {
                    Ok(data) => data,
                    Err(e) => {
                        println!("Error in reading file: {}\nReloading...", e);
                        continue;
                    }
                };

                let mut vm = Machine::new();
                let be_bytes = vm.run(file_data, debug_mode).to_be_bytes();
                println!("\nSuccessfuly ran: {}.nar. \nLast item on vm stack: {}", file_name, u64::from_be_bytes(be_bytes));
            },

            3 => {
                let (file_name, file_data) = match get_file_data(false) {
                    Ok(data) => data,
                    Err(e) => {
                        println!("Error in reading file: {}\nReloading...", e);
                        continue;
                    }
                };

                let mut comp = Compiler::new(format!("nar files/{}.binar", file_name));
                match comp.compile(String::from_utf8_lossy(&file_data), debug_mode) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("\nError in compiling: {:?}.\nReloading...", e);
                        continue
                    }
                }

                let mut vm = Machine::new();
                let be_bytes = vm.run(comp.get_output().clone(), debug_mode).to_be_bytes();
                println!("\nSuccessfuly ran: {}.nar. \nLast item on vm stack: {}", file_name, u64::from_be_bytes(be_bytes));

            }

            _ => {
                println!("Exiting...");
                break
            }
        }
    }
}

fn get_file_data(binary: bool) -> Result<(String, Vec<u8>), String> {
    println!("\nEnter the file name:");

    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read line");

    println!("");

    file_name = file_name.trim().into();

    if binary {
        match fs::read(format!("nar files/{}.binar", file_name)) {
            Ok(data) => Ok((file_name, data)),
            Err(e) => Err(format!("{:?}\n{:?}", file_name, e.to_string()))
        }
    }
    else {
        match fs::read(format!("nar files/{}.nar", file_name)) {
            Ok(data) => Ok((file_name, data)),
            Err(e) => Err(format!("{:?}\n{:?}", file_name, e.to_string()))
        }
    }
}
