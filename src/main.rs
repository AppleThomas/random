// to begin use cargo run [name of file]

mod schedulings;

use std::env;
use std::fs::File;

use schedulings::cpu::CPU;

fn main() {
    // parses command line
    let args: Vec<String> = env::args().collect();

    // checks if number of arguments is correct
    if args.len() != 2 {
        panic!("Use one input file");
    }

    // takes input file name listed in command line
    let input_file_name = &args[1];
    let input_file = File::open(&args[1]);

    // checks if valid file
    let file: File = match input_file {
        Ok(file) => file,
        Err(error) => {
            panic!("Error opening file: {}", error);
        }    
    };

    // reads file into a scheduling struct
    match schedulings::schedule::read_contents(file){
        Ok(schedule_model) => {
            let mut cpu = CPU::default();
            cpu.run(schedule_model);
            let _ = cpu.write_output_file(&input_file_name.replace(".in", ".out"));
        }
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    }
}


