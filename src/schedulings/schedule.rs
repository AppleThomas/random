// umbrella for all schedule algorithms

use core::panic;
use std::error::Error;
use std::fs::File; 
use std::io::{BufReader, BufRead, Lines};

use super::{
    process::Process, 
    fcfs::FCFS, 
    real_sjf::RealSjf, 
    sjf::SJF,
    rr::RR, 
    scheduler::Scheduler, 
};


pub struct ScheduleModel {
    pub number_of_processes: i32,
    pub time_units: i32,
    pub scheduler: Box<dyn Scheduler>,
    pub process_list: Vec<Process>
}

pub fn read_contents(input_file: File) -> Result<ScheduleModel, Box<dyn Error>> {
    // Create a BufReader to efficiently read lines
    let reader: BufReader<File> = BufReader::new(input_file);

    // creates an iterator for the lines of text in the file
    let mut lines_iter: Lines<BufReader<File>> = reader.lines();

    let mut schedule_model = ScheduleModel {
        number_of_processes: 0,
        time_units: 0,
        scheduler: Box::new(FCFS::default()),
        process_list: vec![],
    };

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

    schedule_model.scheduler = parse_scheduler(&mut lines_iter)?;

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

        let new_process = Process::parse(process_line)?;
        schedule_model.process_list.push(new_process);
    }

    if end_flag == false {
        panic!("no end stated");
    }

    Ok(schedule_model)
}


/// Reads the next lines to determine the scheduler
fn parse_scheduler(lines_iter: &mut Lines<BufReader<File>>) -> Result<Box<dyn Scheduler>, Box<dyn Error>> {

    if let Some(Ok(next_line)) = lines_iter.next() {
        if !(next_line.split_whitespace().nth(0).unwrap_or_default() == "use") {
            panic!("incorrect scheduler algorithm line format");
        }

        let scheduler_name = next_line.split_whitespace().nth(1).unwrap_or_default().to_string();
        
        return match scheduler_name.as_str() {
                "fcfs" => Ok(Box::new(FCFS::default())),
                "sjf" => Ok(Box::new(SJF::default())),
                "rr" => parse_rr(lines_iter),
                "realSJF" => Ok(Box::new(RealSjf::default())),
                _ => panic!("Scheduler name not found in file!")
        }
    }

    panic!("no `use` line found")
}

fn parse_rr(lines_iter: &mut Lines<BufReader<File>>) -> Result<Box<dyn Scheduler>, Box<dyn Error>> {

    if let Some(Ok(next_line)) = lines_iter.next() {
        if next_line.split_whitespace().nth(0).unwrap_or_default() == "quantum" {
            let quantum = next_line.split_whitespace().nth(1).unwrap_or_default().parse::<i32>()?;
            return Ok(Box::new(RR::new(quantum)));
        }
    }

    panic!("quantum line not found!");
}