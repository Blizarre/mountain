use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Default)]
pub struct Stats {
    num_events: u32,
    total_time: usize,
}

impl Stats {
    pub fn add(self: &mut Self, time: usize) {
        self.num_events += 1;
        self.total_time += time;
    }
    pub fn avg(self: &Self) -> f32 {
        self.total_time as f32 / self.num_events as f32
    }
}

impl Display for Stats {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(
            fmt,
            "time/events: {:.1} ms, # of events: {}, total time: {} ms",
            self.avg(),
            self.num_events,
            self.total_time
        )
    }
}
