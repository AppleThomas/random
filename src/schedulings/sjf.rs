use core::time;
use std::collections::HashMap;

use super::process::{Process, ProcessState};
use super::scheduler::Scheduler;

#[derive(Default)]
pub struct SJF {
    /// A hashmap that contains keys that represent the process name and a integer for time remaining
    pub job_list: HashMap<String, (i32, i32)>,
    pub shortest_job: Option<String>,
}

impl SJF {
    /// Iterate through the job_list to find the one with the shortest value
    /// Sets the shortest_job to the key of that pair
    fn find_shortest_job(&mut self) {
        let mut shortest = ("", &i32::MAX);
        let mut shortest_arrival = i32::MAX;

        for (name, (time_left, arrival_time)) in self.job_list.iter() {
            if shortest.1 > time_left || (shortest.1 == time_left && *arrival_time < shortest_arrival) {
                shortest = (name, time_left);
                shortest_arrival = *arrival_time;
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

impl Scheduler for SJF {
    fn descriptive_name(&self) -> String {
        "preemptive Shortest Job First".to_string()
    }
    
    fn on_arrive(&mut self, process: &mut Process, time: i32) {
        self.job_list.insert(process.name.to_owned(), (process.time_remaining, process.arrival_time));

        if let Some(shortest_job_name) = &self.shortest_job {
            if self.job_list[shortest_job_name].0 <= process.time_remaining {
                return;
            }
        }

        self.shortest_job = Some(process.name.to_owned());
        process.select(time);
    }

    fn on_tick(&mut self, process: &mut Process, _: i32) {
        self.job_list.insert(process.name.to_owned(), (process.time_remaining, process.arrival_time));
    }

    fn pre_tick(&mut self, process: &mut Process, time: i32) {
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

    fn on_finish(&mut self, finished_process: &Process, _: i32) {
        self.job_list.remove(&finished_process.name);
        self.find_shortest_job();
    }

    fn selected_process_name(&self) -> Option<String> {
        match &self.shortest_job {
            Some(job_name) => Some(job_name.to_string()),
            None => None
        }
    }
}