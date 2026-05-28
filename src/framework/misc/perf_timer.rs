// 对应 C++ 中的 PerfTimer.h / PerfTimer.cpp
// 性能计时器

use std::time::Instant;

pub struct PerfTimer {
    start: Instant,
    elapsed: f64,
    running: bool,
}

impl PerfTimer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            elapsed: 0.0,
            running: false,
        }
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
        self.running = true;
    }

    pub fn stop(&mut self) -> f64 {
        if self.running {
            self.elapsed = self.start.elapsed().as_secs_f64();
            self.running = false;
        }
        self.elapsed
    }

    pub fn get_elapsed(&self) -> f64 {
        if self.running {
            self.start.elapsed().as_secs_f64()
        } else {
            self.elapsed
        }
    }
}
