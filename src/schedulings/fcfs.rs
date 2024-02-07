
use std::{collections::HashMap, vec};

use super::scheduler::Scheduler;
use super::process::{Process, ProcessState};


#[derive(Default)]

pub struct FCFS {
    /// A hashmap that contains keys that represent the process name and a integer for arrival time
    pub job_list: HashMap<String, i32>,
    pub first_job: Option<String>
}

pub struct FcfsProcess {
    pub name: String,
    pub arrival_time: i32,
}

impl FCFS {
    fn find_first_job(&mut self) {
        let mut first = ("", &i32::MAX);

        for (name, arrival_time) in self.job_list.iter() {
            if first.1 > arrival_time {
                first = (name, arrival_time)
            }
        }

        if first.0 == "" {
            self.first_job = None;
        }
        else {
            self.first_job = Some(first.0.to_string())
        }
    }
}

impl Scheduler for FCFS {

    fn descriptive_name(&self) -> String {
        "First-Come First-Served".to_string()
    }

    fn on_arrive(&mut self, process: &mut super::process::Process, time: i32) {
        
        self.job_list.insert(process.name.to_owned(), process.arrival_time);

        if let Some(first_job_name) = &self.first_job {
            if self.job_list[first_job_name] <= process.arrival_time {
                return;
            }
        }

        self.first_job = Some(process.name.to_owned());
        process.select(time);

    }

    fn pre_tick(&mut self, process: &mut super::process::Process, time: i32) {

        if let Some(name) = &self.first_job {

            if name == &process.name {
                if process.state != Some(ProcessState::Running) {
                    process.select(time);
                }
            }
            else {
                process.deselect();
            }
        }
    }

    fn on_tick(&mut self, process: &mut super::process::Process, time: i32) {
        // do nothing
    }

    
    fn on_finish(&mut self, finished_process: &Process, current_time: i32) {
        
        self.job_list.remove(&finished_process.name);
        self.find_first_job();

    }

    fn selected_process_name(&self) -> Option<String> {
        // println!("name is {:?}", self.first_job);
        match &self.first_job {
            Some(job_name) => Some(job_name.to_string()),
            None => None
        }
    }

    
}