
use std::collections::HashMap;

use super::scheduler::Scheduler;
use super::process::{Process, ProcessState};


#[derive(Default)]

pub struct RealSjf {
    /// A hashmap that contains keys that represent the process name and a integer for burst time
    pub job_list: HashMap<String, i32>,
    pub shortest_job: Option<String>
}


impl RealSjf {
    fn find_shortest_job(&mut self) {
        let mut shortest = ("", &i32::MAX);

        for (name, burst_time) in self.job_list.iter() {
            
            if shortest.1 > burst_time {
                
                shortest = (name, burst_time)
            }
        }

        if shortest.0 == "" {
            self.shortest_job = None;
        }
        else {
            self.shortest_job = Some(shortest.0.to_string())
        }
    }
}

impl Scheduler for RealSjf {

    fn descriptive_name(&self) -> String {
        "real Shortest Job First (non-preemptive)".to_string()
    }

    fn on_arrive(&mut self, process: &mut super::process::Process, time: i32) {
        
        self.job_list.insert(process.name.to_owned(), process.burst_time);

        // if let Some(first_job_name) = &self.first_job {
        //     if self.job_list[first_job_name] <= process.arrival_time {
        //         return;
        //     }
        // }

        if self.job_list.len() == 1
        {
            self.shortest_job = Some(process.name.to_owned());
            process.select(time);
        }

    }

    fn pre_tick(&mut self, process: &mut super::process::Process, time: i32) {

        if let Some(name) = &self.shortest_job {

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

    fn on_tick(&mut self, _process: &mut super::process::Process, _time: i32) {
        // do nothing
    }

    
    fn on_finish(&mut self, finished_process: &Process, _current_time: i32) {

        let removed_value = self.job_list.remove(&finished_process.name);

        if let Some(_value) = removed_value {
            self.find_shortest_job();
        }

    }

    fn selected_process_name(&self) -> Option<String> {
        match &self.shortest_job {
            Some(job_name) => Some(job_name.to_string()),
            None => None
        }
    }

    
}