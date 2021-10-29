``` text
┌─┐┬ ┬┌┐┌┌─┐┌─┐┬ ┬┌─┐┌┬┐┬ ┬┬  ┌─┐┬─┐
├┤ │ ││││└─┐│  ├─┤├┤  │││ ││  ├┤ ├┬┘
└  └─┘┘└┘└─┘└─┘┴ ┴└─┘─┴┘└─┘┴─┘└─┘┴└─
```
![crates.io](https://img.shields.io/crates/v/funscheduler.svg "crates.io")

Time based function execution scheduler

``` rust
use funscheduler::{FunScheduler, Timing};

fn main() {
    // Execute job every five seconds
    FunScheduler::interval(job, Timing::Seconds(5));
}

fn job() {
    println!("Hello, world");
}
```

## Time configurations with the Timing Enum

``` rust
Timing::Seconds(1)
Timing::Minutes(25)
Timing::Hours(2)
Timing::Days(1)
```

## Job runners, different methods to execute the function

``` rust
// Evaluates a function at specified intervals, starting now
FunScheduler::interval(job, Timing::Seconds(1))

// Evaluates a function at specified intervals, does not execute
// the function immedialy
FunScheduler::rinterval(job, Timing::Seconds(1))

// Execute function once after a specified amount of time
FunScheduler::after(job, Timing::Seconds(1))
```

⧉
