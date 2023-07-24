use std::time::Instant;
use crate::building_poset;
use crate::compute_mobius;

pub struct Stopwatch {
    start_time: Instant,
}

impl Stopwatch {
    pub fn new() -> Self {
        Stopwatch {
            start_time: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn stop(&self) -> f64 {
        let elapsed_time = Instant::now().duration_since(self.start_time);
        let seconds = elapsed_time.as_secs_f64();
        seconds / 100_000_000.0
    }
}


pub fn benchmark(x: Vec<u8>, y: Vec<u8>, itterations: u32) -> f64{
    let mut average: f64 = 0.0;
    let mut stopwatch = Stopwatch::new();
    
    for _ in 0..itterations{
        stopwatch.start();
        let poset = building_poset::build_poset(&y, &x);
        let result = compute_mobius::compute(&y, &x, &poset);
        average += stopwatch.stop();
    }
    return average / itterations as f64;
}