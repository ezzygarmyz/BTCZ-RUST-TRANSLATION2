use std::time::{Duration, Instant};

pub struct PerfTimer {
    start: Instant,
}

impl PerfTimer {
    pub fn new() -> Self {
        PerfTimer {
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

pub fn measure_perf<F: Fn()>(name: &str, function: F) {
    let timer = PerfTimer::new();
    function();
    let elapsed = timer.elapsed();
    println!("{}: {:?}", name, elapsed);
}
