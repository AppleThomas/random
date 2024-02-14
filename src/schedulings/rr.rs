use std::collections::VecDeque;

use super::process::Process;
use super::scheduler::Scheduler;

/// Struct representing the Round-Robin scheduler
pub struct RR {
    quantum: i32,
    quantum_remaining: i32,

    process_queue: VecDeque<String>,
    selected_process: Option<String>
}

impl RR {
    pub fn new(quantum: i32) -> RR {
        RR {
            quantum,
            quantum_remaining: quantum,
            process_queue: VecDeque::new(),
            selected_process: None
        }
    }
}

impl Scheduler for RR {
    fn descriptive_name(&self) -> String {
        format!("Round-Robin\nQuantum {:3}\n", self.quantum).to_string()
    }
    
    fn on_arrive(&mut self, process: &mut Process, _: i32) {
        self.process_queue.push_back(process.name.to_owned());
    }

    fn on_tick(&mut self, process: &mut Process, _: i32) {
        if let Some(selected_name) = &self.selected_process {
            if selected_name == &process.name {
                self.quantum_remaining -= 1;

                if self.quantum_remaining == 0 {
                    process.deselect();
                    self.selected_process = None;
                    self.process_queue.push_back(process.name.to_owned());
                }
            }
        }
    }

    fn pre_tick(&mut self, process: &mut Process, time: i32) {
        if self.selected_process.is_some() { return; } // don't do any selecting if I already have a selected process

        if let Some(front_name) = self.process_queue.front() {
            if front_name == &process.name { // if the pre_ticking process is next to be selected (select it)
                self.selected_process = self.process_queue.pop_front();
                process.select(time);
                self.quantum_remaining = self.quantum;
            }
        }
    }

    fn on_finish(&mut self, _: &Process, _: i32) {
        self.selected_process = None;
    }

    fn selected_process_name(&self) -> Option<String> {
        match &self.selected_process {
            Some(name) => Some(name.to_string()),
            None => None
        }
    }
}