use super::process::Process;

pub trait Scheduler {
    /// Runs when the given process arrives on the cpu at the given time
    fn on_arrive(&mut self, process: &mut Process, time: i32);

    /// Runs just before the proces ticks before the given time
    fn pre_tick(&mut self, process: &mut Process, time: i32);

    /// Runs just after the process ticks into the given time (without finishing)
    fn on_tick(&mut self, process: &mut Process, time: i32);

    /// Runs just after the process ticks into the given time and finishes
    fn on_finish(&mut self, process: &Process, time: i32);

    // The name of the scheduler's currently selected process
    fn selected_process_name(&self) -> Option<String>;

    /// The longer name of the scheduler, used when printing outputs
    fn descriptive_name(&self) -> String;
}