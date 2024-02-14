use std::{fs::File, io::{Result, Write}};

use super::{
    process::Process, 
    schedule::ScheduleModel, 
    scheduler::Scheduler, 
};

#[derive(Default)]
pub struct CPU {
    output: Vec<String>,
}

impl CPU {
    pub fn run(&mut self, mut model: ScheduleModel) {
        let scheduler = model.scheduler.as_mut();

        self.output.push(format!("{:3} processes", model.number_of_processes));
        self.output.push(format!("Using {}", scheduler.descriptive_name()));
        self.output.push(format!(""));

        for t in 0..model.time_units {
            self.tick_proceses(&mut model.process_list, t);
            self.handle_on_tick(&mut model.process_list, scheduler, t);
            self.handle_finishes(&mut model.process_list, scheduler, t);
            self.handle_arrivals(&mut model.process_list, scheduler, t);
            self.handle_pre_tick(&mut model.process_list, scheduler, t);

            self.handle_selection_output(&model.process_list, scheduler, t)
        }

        self.output.push(format!("Finished at time {}", model.time_units));
        self.output.push(format!(""));

        self.handle_status_output(&model.process_list);
    }

    fn tick_proceses(&mut self, processes: &mut Vec<Process>, cur_time: i32) {
        processes.iter_mut()
        .filter(|p| p.arrived(cur_time - 1))
        .for_each(|p| {
            p.tick(cur_time);
        });
    }


    fn handle_on_tick(&mut self, processes: &mut Vec<Process>, scheduler: &mut dyn Scheduler, cur_time: i32) {
        processes.iter_mut()
        .filter(|p| !p.finished() && p.arrived(cur_time - 1))
        .for_each(|p| scheduler.on_tick(p, cur_time));
    }

    fn handle_finishes(&mut self, processes: &mut Vec<Process>, scheduler: &mut dyn Scheduler, cur_time: i32) {
        processes.iter_mut()
        .filter(|p| p.finished() && p.finish_time == cur_time)
        .for_each(|p| {
            self.output.push(format!("Time {:3} : {} finished", cur_time, p.name));
            scheduler.on_finish(p, cur_time);
        });
    }

    fn handle_arrivals(&mut self, processes: &mut Vec<Process>, scheduler: &mut dyn Scheduler, cur_time: i32) {
        // goes through all the processes inputted in and sees if the current time in the scheduling matches any arrival times
        for process in processes.iter_mut() {
            if process.arrival_time == cur_time {
                process.deselect(); // sets the process as ready when arriving
                self.output.push(format!("Time {:3} : {} arrived", cur_time, process.name));
                scheduler.on_arrive(process, cur_time);
            }
        }
    }

    fn handle_pre_tick(&mut self, processes: &mut Vec<Process>, scheduler: &mut dyn Scheduler, cur_time: i32) {
        processes.iter_mut()
        .filter(|p| !p.finished() && p.arrived(cur_time))
        .for_each(|p| {
            scheduler.pre_tick(p, cur_time);
        });
    }

    fn handle_selection_output(&mut self, processes: &Vec<Process>, scheduler: &dyn Scheduler, cur_time: i32) {    
        // if we have a possible new selection at the end of the cur_time
        for process in processes {
            if let Some(last_selection_time) = process.last_selection_time {
                if last_selection_time == cur_time {
                    self.output.push(format!("Time {:3} : {} selected (burst {:3})", cur_time, process.name, process.time_remaining))
                }
            }
        }

        if scheduler.selected_process_name().is_none() {
            self.output.push(format!("Time {:3} : Idle", cur_time));
        }
    }

    fn handle_status_output(&mut self, processes: &Vec<Process>) {
        for process in processes.iter() {
            if process.finished() {
                let line = format!("{} wait {:3} turnaround {:3} response {:3}", process.name, process.wait_time, process.turnaround_time, process.response_time);
                self.output.push(line);
            }
            else {
                self.output.push(format!("{} did not finish", process.name));
            }
        }
    }


    pub fn print_output(&self) {
        for line in self.output.iter() {
            println!("{}", line);
        }
    }

    pub fn write_output_file(&self, file_path: &str) -> Result<()>{
        let mut file = File::create(file_path)?;

        for line in self.output.iter() {
            file.write((line.to_owned() + "\n").as_bytes())?;
        }

        Ok(())
    }

}