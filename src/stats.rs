use core::fmt;
use std::fmt::{Display, Formatter};
use std::time::{Instant, Duration};

pub struct Stats {
    num_events: u32,
    total_time: Duration,
    last_tick: Instant
}

impl Default for Stats {
    fn default() -> Self {
        Self{num_events: 0, total_time: Duration::default(), last_tick: Instant::now()}
    }
}

impl Stats {

    /// Mark the end of the event and return the time elapsed during this event
    pub fn end_event(self: &mut Self) -> u32{
        let elapsed = Instant::now() - self.last_tick;
        self.num_events += 1;
        self.total_time += elapsed;
        elapsed.subsec_millis()
    }

    pub fn start_event(self: &mut Self) {
        self.last_tick = Instant::now();
    }

    pub fn time<F>(self: &mut Self, mut f: F) where F: FnMut() {
        let before = Instant::now();
        f();
        self.total_time += Instant::now().duration_since(before);
        self.num_events += 1;
    }

    pub fn avg_micro(self: &Self) -> f32 {
        self.total_time.subsec_micros() as f32 / self.num_events as f32
    }
}

impl Display for Stats {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(
            fmt,
            "time/events: {:.2} ms, # of events: {}, total time: {} ms",
            self.avg_micro() / 1000.0,
            self.num_events,
            self.total_time.subsec_millis() / 1000
        )
    }
}
