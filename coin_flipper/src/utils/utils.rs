use std::time::{Duration, Instant};

pub fn round_to(value: f64, places: i32) -> f64 {
    let multiplier = 10f64.powi(places);
    (value * multiplier).round() / multiplier
}

pub fn time_execution<F, R>(func: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();
    (result, duration)
}
