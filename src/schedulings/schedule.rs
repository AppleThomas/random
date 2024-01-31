// umbrella for all schedule algorithms

use core::panic;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Default)]
pub struct ScheduleModel {
    pub number_of_processes: i32,
    pub time_units: i32,
    pub schedule_algorithm: String,
    pub process_list: Vec<Process>

}

#[derive(Default)]
pub struct Process {
    pub process_name: String,
    pub arrival_time: i32,
    pub amount_till_burst: i32,
}

pub fn read_contents(input_file: File) -> io::Result<ScheduleModel>{
    // Create a BufReader to efficiently read lines
    let reader: io::BufReader<File> = io::BufReader::new(input_file);

    // creates an iterator for the lines of text in the file
    let mut lines_iter: io::Lines<io::BufReader<File>> = reader.lines();

    let mut schedule_model = ScheduleModel::default();

    // reads first line for process count
    if let Some(Ok(first_line)) = lines_iter.next() {

        // makes sure we're actually getting process count
        if !(first_line.split_whitespace().nth(0).unwrap_or_default() == "processcount") {
            panic!("failed to get process count");
        }

        // gets the processcount number
        if let Ok(process_count) = first_line.split_whitespace().nth(1).unwrap_or_default().parse::<i32>() {
            schedule_model.number_of_processes = process_count;
        } else {
            panic!("failed to get process count");
        }
        
    } else {
        panic!("no first line found")
    }

    // get runfor next
    if let Some(Ok(second_line)) = lines_iter.next() {

        // makes sure we're actually getting process count
        if !(second_line.split_whitespace().nth(0).unwrap_or_default() == "runfor") {
            panic!("failed to get runfor");
        }

        if let Ok(runfor) = second_line.split_whitespace().nth(1).unwrap_or_default().parse::<i32>() {
            schedule_model.time_units = runfor;
        } else {
            panic!("failed to get runfor");
        }
        
    } else {
        panic!("no second line found")
    }

    // get which algo to use next
    if let Some(Ok(third_line)) = lines_iter.next() {

        // makes sure we're actually getting process count
        if !(third_line.split_whitespace().nth(0).unwrap_or_default() == "use") {
            panic!("failed to get runfor");
        }

        schedule_model.schedule_algorithm = third_line.split_whitespace().nth(1).unwrap_or_default().to_string();
        
    } else {
        panic!("no second line found")
    }

    let mut end_flag = false;

    // // Iterate over the rest of the lines and process each line
    for line in lines_iter {

        // uses ? to extract the string value from the Result<String, Error> type in line variable
        let line = line?;
        
        let mut process_line = line.split_whitespace();

        
        

        // we only need to read the first line to know what action to take so we'll use a reference cause we don't want to waste power on cloning
        if let Some(first_word) = process_line.nth(0) {
            if first_word == "end" {
                end_flag = true;
                break;
            }
            else if first_word != "process" {
                panic!("improper formatting or out of place line")
            }
            
        } else {
            panic!("No lines");
        }

        // consumes "name" word
        process_line.next();


        let mut new_process = Process::default();

        new_process.process_name = process_line.nth(0).unwrap_or_default().to_string();

        // consumes "Arrival" word
        process_line.next();

        // get arrival time of process
        match process_line.nth(0).unwrap_or_default().parse::<i32>() {
            Ok(parsed_value) => {
                new_process.arrival_time = parsed_value;
            }
            Err(_) => {
                
                println!("Failed to parse as i32");
            }
        }

         // consumes "Burst" word
         process_line.next();

         // get burst time of process
        match process_line.nth(0).unwrap_or_default().parse::<i32>() {
            Ok(parsed_value) => {
                new_process.amount_till_burst = parsed_value;
            }
            Err(_) => {
                println!("Failed to parse as i32");
            }
        }

        schedule_model.process_list.push(new_process);
    
    }

    if end_flag == false {
        panic!("no end stated");
    }

    Ok(schedule_model)
}