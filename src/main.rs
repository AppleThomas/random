// to begin use cargo run [name of file]

mod schedulings;

use std::env;
use std::fs::File;

use schedulings::cpu;

fn main() {
    // parses command line
    let args: Vec<String> = env::args().collect();

    // checks if number of arguments is correct
    if args.len() != 2 {
        panic!("Use one input file");
    }

    // takes input file name listed in command line
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
            // println!("processcount is {}", schedule_model.number_of_processes);
            // println!("runfor is {}", schedule_model.time_units);
            // println!("use is {}", schedule_model.schedule_algorithm);
            // for process in schedule_model.process_list {
            //     println!("process name is {} arrival is {} burst is {}", process.process_name, process.arrival_time, process.burst_time);
            // }
            cpu::run(schedule_model);
        }
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    }
}


