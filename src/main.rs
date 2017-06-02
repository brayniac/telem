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
mod sources;

use logging::set_log_level;
use metrics::Collector;
use options::{PROGRAM, VERSION};

fn main() {
    let options = options::init();

    // initialize logging
    set_log_level(options.opt_count("verbose"));
    info!("{} {}", PROGRAM, VERSION);

    // initialize metrics
    // let mut metrics = metrics::init(options.opt_str("listen"));
    // let clock = metrics.get_clocksource();
    // let mut sender = metrics.get_sender();
    // thread::spawn(move || { metrics.run(); });

    let mut collector = Collector::new(options.opt_str("listen"));

    loop {
        collector.collect();
    }
}
