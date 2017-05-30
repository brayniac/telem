#[macro_use]
extern crate log;
extern crate getopts;
extern crate perfcnt;
extern crate tic;
extern crate shuteye;
extern crate x86;

mod logging;
mod metrics;
mod options;

use logging::set_log_level;
use options::{PROGRAM, VERSION};
use perfcnt::AbstractPerfCounter;
use std::thread;
use std::time::Duration;
use tic::Sample;

fn main() {
    let options = options::init();

    // initialize logging
    set_log_level(options.opt_count("verbose"));
    info!("{} {}", PROGRAM, VERSION);

    // initialize metrics
    let mut metrics = metrics::init(options.opt_str("metrics"));
    let clock = metrics.get_clocksource();
    let mut sender = metrics.get_sender();
    thread::spawn(move || { metrics.run(); });

    let mut counters = metrics::intel_core_counters();

    loop {
        let t0 = clock.counter();
        for (_, counter) in counters.iter_mut() {
            counter.start().expect("Could not start perf counter");
        }
        shuteye::sleep(Duration::new(0, 1_000_000));
        let t1 = clock.counter();
        for (_, counter) in counters.iter_mut() {
            counter.stop().expect("Could not start perf counter");
        }
        for (metric, counter) in counters.iter_mut() {
            let _ = sender
                .send(Sample::counted(t0,
                                      t1,
                                      counter.read().expect("Could not read perf counter"),
                                      metric.clone()));
        }
        for (_, counter) in counters.iter_mut() {
            counter.reset().expect("Could not reset perf counter");
        }
    }
}
