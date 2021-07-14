// .---------------------------------------------------------------.
// |           Written and placed in the public domain by           |
// |        Jackson G. Kaindume<seestem@protonmail.com>  ⧉          |
// '----------------------------------------------------------[2021]+
//! # Time based function execution scheduler

use std::time::Duration;

pub enum Timing {
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
}

pub struct FunScheduler;

impl FunScheduler {
    /// Execute a function in specified time intervals, starting now.
    pub fn interval(job: fn(), timing: Timing) {
        let time_control = calc_time(timing);

        loop {
            job();
            std::thread::sleep(time_control);
        }
    }

    /// Execute function once after a specified amount of time
    pub fn after(job: fn(), timing: Timing) {
        let time_control = calc_time(timing);
        std::thread::sleep(time_control);
        job();
    }
}

pub fn calc_time(timing: Timing) -> Duration {
    let final_seconds;
    match timing {
        Timing::Seconds(seconds) => final_seconds = Duration::from_secs(seconds),
        Timing::Minutes(minutes) => {
            let seconds = minutes * 60;
            final_seconds = Duration::from_secs(seconds);
        }
        Timing::Hours(hours) => {
            let seconds = hours * 3600;
            final_seconds = Duration::from_secs(seconds);
        }
        Timing::Days(days) => {
            let seconds = days * 86_400;
            final_seconds = Duration::from_secs(seconds);
        }
    }

    final_seconds
}
