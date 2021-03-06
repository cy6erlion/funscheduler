//! # funscheduler
//! Time based function execution scheduler
use std::time::Duration;

/// Timing configuration
pub enum Timing {
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
}

/// Different methods for running functions according to time.
pub struct FunScheduler;

impl FunScheduler {
    /// Execute a function in specified time interval,
    /// the function will be executed imidiately.
    pub fn interval(job: fn(), timing: Timing) {
        let time = calc_time(timing);

        loop {
            std::thread::spawn(move || {
                job();
            });

            std::thread::sleep(time);
        }
    }

    /// Like interval but does not execute immediately.
    pub fn rinterval(job: fn(), timing: Timing) {
        let time = calc_time(timing);

        loop {
            std::thread::sleep(time);

            std::thread::spawn(move || {
                job();
            });
        }
    }

    /// Execute function once after a specified amount of time
    pub fn after(job: fn(), timing: Timing) {
        std::thread::sleep(calc_time(timing));
        job();
    }
}

/// Calculate time
fn calc_time(timing: Timing) -> Duration {
    match timing {
        Timing::Seconds(s) => Duration::from_secs(s),
        Timing::Minutes(minutes) => Duration::from_secs(minutes * 60),
        Timing::Hours(hours) => Duration::from_secs(hours * 3600),
        Timing::Days(days) => Duration::from_secs(days * 86_400),
    }
}
