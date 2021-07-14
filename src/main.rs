use funscheduler::{FunScheduler, Timing};

fn main() {
    // Execute job every second
    //FunScheduler::interval(job, Timing::Seconds(1));

    // Execute job once after 5 seconds
    FunScheduler::after(job, Timing::Seconds(5));
}

fn job() {
    println!("Hello, world");
}
