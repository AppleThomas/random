use super::{
    schedule::ScheduleModel, 
    scheduler::Scheduler,
    sjf::SJF,
    process::Process
};

pub fn run(mut model: ScheduleModel) {
    let mut boxed_scheduler = parse_scheduler(&model.schedule_algorithm).expect("Could not parse scheduler");
    let scheduler = boxed_scheduler.as_mut();

    println!("{:3} processes", model.number_of_processes);
    println!("Using {}", scheduler.descriptive_name());

    for t in 0..model.time_units {
        tick_proceses(&mut model.process_list, t);
        handle_on_tick(&mut model.process_list, scheduler, t);
        handle_finishes(&mut model.process_list, scheduler, t);
        handle_arrivals(&mut model.process_list, scheduler, t);
        handle_pre_tick(&mut model.process_list, scheduler, t);

        if scheduler.selected_process_name().is_none() {
            println!("Time {:3} : Idle", t)
        }
    }

    println!("Finished at time {}", model.time_units);
    println!();

    for process in model.process_list.iter() {
        process.print_status();
    }
}

fn tick_proceses(processes: &mut Vec<Process>, cur_time: i32) {
    processes.iter_mut()
    .filter(|p| p.arrived(cur_time - 1))
    .for_each(|p| {
        p.tick(cur_time);
    });
}

fn handle_on_tick(processes: &mut Vec<Process>, scheduler: &mut dyn Scheduler, cur_time: i32) {
    processes.iter_mut()
    .filter(|p| !p.finished() && p.arrived(cur_time - 1))
    .for_each(|p| scheduler.on_tick(p, cur_time));
}

fn handle_finishes(processes: &mut Vec<Process>, scheduler: &mut dyn Scheduler, cur_time: i32) {
    processes.iter_mut()
    .filter(|p| p.finished())
    .for_each(|p| scheduler.on_finish(p, cur_time));
}

fn handle_arrivals(processes: &mut Vec<Process>, scheduler: &mut dyn Scheduler, cur_time: i32) {
    for process in processes.iter_mut() {
        if process.arrival_time == cur_time {
            process.deselect(); // sets the process as ready when arriving
            println!("Time {:3} : {} arrived", cur_time, process.name);
            scheduler.on_arrive(process, cur_time);
        }
    }
}

fn handle_pre_tick(processes: &mut Vec<Process>, scheduler: &mut dyn Scheduler, cur_time: i32) {
    processes.iter_mut()
    .filter(|p| !p.finished() && p.arrived(cur_time))
    .for_each(|p| {
        scheduler.pre_tick(p, cur_time);
    });
}

/// Attempts to convert the given string slice to the correct scheduler struct
fn parse_scheduler(name: &str) -> Option<Box<dyn Scheduler>> {
    match name {
        "fcfs" => todo!(),
        "sjf" => Some(Box::new(SJF::default())),
        "rr" => todo!(),
        _ => None
    }
}