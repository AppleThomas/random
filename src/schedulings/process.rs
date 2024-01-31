pub enum ProcessState {
    Ready,
    Running,
}

#[derive(Default)]
pub struct Process {
    pub process_name: String,
    pub arrival_time: i32,
    pub burst_time: i32,
    pub time_remaining: i32,
    pub state: Option<ProcessState>,

    pub turnaround_time: i32,
    pub response_time: i32,
    pub wait_time: i32
}

impl Process {
    pub fn new(process_name: String, arrival_time: i32, burst_time: i32) -> Process {
        Process {
            process_name,
            arrival_time,
            burst_time,
            time_remaining: burst_time,
            state: None,
            turnaround_time: 0,
            response_time: 0,
            wait_time: 0
        }
    }

    // runs when time advances by 1 
    // returns whether process finished in that time
    pub fn tick(&mut self) {
        if let Some(state) = &self.state {
            self.turnaround_time += 1;
            match state {
                ProcessState::Ready => {
                    self.wait_time += 1;
                    
                    // if we have not been selected before
                    if self.burst_time == self.time_remaining {
                        self.response_time += 1;
                    }
                },
                ProcessState::Running => {
                    self.time_remaining -= 1;

                    if self.time_remaining == 0 {
                        self.state = None;
                    }
                },
            }
        }
    }

    pub fn finished(&self) -> bool {
        self.time_remaining == 0
    }
}