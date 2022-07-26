use core::fmt;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

pub struct Stats {
    num_events: u32,
    total_time: Duration,
    last_tick: Instant,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            num_events: 0,
            total_time: Duration::default(),
            last_tick: Instant::now(),
        }
    }
}

impl Stats {
    /// Mark the end of the event and return the time elapsed during this event
    pub fn end_event(&mut self) -> u32 {
        let elapsed = self.last_tick.elapsed();
        self.num_events += 1;
        self.total_time += elapsed;
        elapsed.subsec_millis()
    }

    pub fn start_event(&mut self) {
        self.last_tick = Instant::now();
    }

    pub fn time<F>(&mut self, mut f: F)
    where
        F: FnMut(),
    {
        let before = Instant::now();
        f();
        self.total_time += before.elapsed();
        self.num_events += 1;
    }

    pub fn avg_micro(&self) -> f32 {
        self.total_time.as_micros() as f32 / self.num_events as f32
    }
}

impl Display for Stats {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(
            fmt,
            "time/events: {:.2} ms, # of events: {}, total time: {} ms",
            self.avg_micro() / 1000.0,
            self.num_events,
            self.total_time.as_millis()
        )
    }
}
