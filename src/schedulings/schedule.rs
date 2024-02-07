// umbrella for all schedule algorithms

use core::panic;
use std::error::Error;
use std::fs::File; 
use std::io::{BufReader, BufRead, Lines};

use super::process::Process;

#[derive(Default)]
pub struct ScheduleModel {
    pub number_of_processes: i32,
    pub time_units: i32,
    pub schedule_algorithm: String,
    pub process_list: Vec<Process>
}

pub fn read_contents(input_file: File) -> Result<ScheduleModel, Box<dyn Error>> {
    // Create a BufReader to efficiently read lines
    let reader: BufReader<File> = BufReader::new(input_file);

    // creates an iterator for the lines of text in the file
    let mut lines_iter: Lines<BufReader<File>> = reader.lines();

    let mut schedule_model = ScheduleModel::default();

    // reads first line for process count
    if let Some(Ok(first_line)) = lines_iter.next() {
        // makes sure we're actually getting process count
        if !(first_line.split_whitespace().nth(0).unwrap_or_default() == "processcount") {
            panic!("failed to get process count");
        }

        // gets the number of processes
        schedule_model.number_of_processes = first_line.split_whitespace().nth(1).unwrap_or_default().parse::<i32>()?;
        
    } else {
        panic!("no first line found")
    }

    // get runfor next
    if let Some(Ok(second_line)) = lines_iter.next() {

        // makes sure we're actually getting process count
        if !(second_line.split_whitespace().nth(0).unwrap_or_default() == "runfor") {
            panic!("failed to get runfor");
        }

        schedule_model.time_units = second_line.split_whitespace().nth(1).unwrap_or_default().parse::<i32>()?;
        
    } else {
        panic!("no second line found")
    }

    // get which algo to use next
    if let Some(Ok(third_line)) = lines_iter.next() {

        // makes sure we're actually getting process count
        if !(third_line.split_whitespace().nth(0).unwrap_or_default() == "use") {
            panic!("failed to get algorithm to use");
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

        // we only need to read the first line to know what action to take so we'll use a reference cause we don't want to waste memory on cloning
        let first_word = process_line.nth(0).expect("No lines");
        
        if first_word == "end" {
            end_flag = true;
            break;
        }
        else if first_word != "process" {
            panic!("improper formatting or out of place line")
        }

        let new_process = Process::parse(process_line).expect("Error parsing process line!");
        schedule_model.process_list.push(new_process);
    }

    if end_flag == false {
        panic!("no end stated");
    }

    Ok(schedule_model)
}