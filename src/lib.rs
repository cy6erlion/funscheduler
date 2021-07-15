// .---------------------------------------------------------------.
// |           Written and placed in the public domain by           |
// |        Jackson G. Kaindume<seestem@protonmail.com>  ⧉         |
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
    /// Execute a function in specified time interval,
    /// the function will be executed imidiately and then start
    pub fn interval(job: fn(), timing: Timing) {
        let time = calc_time(timing);

        loop {
            std::thread::spawn(move || {
                job();
            });

            std::thread::sleep(time);
        }
    }

    /// Like intervals but does not execute immediately.
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
        Timing::Seconds(s) => return Duration::from_secs(s),
        Timing::Minutes(minutes) => {
            return Duration::from_secs(minutes * 60);
        }
        Timing::Hours(hours) => {
            return Duration::from_secs(hours * 3600);
        }
        Timing::Days(days) => {
            return Duration::from_secs(days * 86_400);
        }
    }
}
