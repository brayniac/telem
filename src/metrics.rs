use perfcnt::AbstractPerfCounter;
use shuteye;
use sources::*;
use std::fmt;
use std::thread;
use std::time::Duration;
use tic::{Clocksource, Interest, Receiver, Sample, Sender};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Metric {
    MemoryLoadsL1Hit,
    MemoryLoadsL1Miss,
    MemoryLoadsL2Hit,
    MemoryLoadsL2Miss,
    MemoryLoadsL3Hit,
    MemoryLoadsL3Miss,
    MemoryLoads,
    MemoryLoadsSplit,
    MemoryStores,
    MemoryStoresSplit,
    UopsRetired,
    Instructions,
    Cycles,
    PageFaults,
    ContextSwitches,
    CpuRunningMilliseconds(String),
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Metric::MemoryLoadsL1Hit => write!(f, "memory_loads_l1_hit"),
            Metric::MemoryLoadsL1Miss => write!(f, "memory_loads_l1_miss"),
            Metric::MemoryLoadsL2Hit => write!(f, "memory_loads_l2_hit"),
            Metric::MemoryLoadsL2Miss => write!(f, "memory_loads_l2_miss"),
            Metric::MemoryLoadsL3Hit => write!(f, "memory_loads_l3_hit"),
            Metric::MemoryLoadsL3Miss => write!(f, "memory_loads_l3_miss"),
            Metric::MemoryLoads => write!(f, "memory_loads"),
            Metric::MemoryStores => write!(f, "memory_stores"),
            Metric::MemoryLoadsSplit => write!(f, "memory_loads_split"),
            Metric::MemoryStoresSplit => write!(f, "memory_stores_split"),
            Metric::UopsRetired => write!(f, "uops_retired"),
            Metric::Instructions => write!(f, "instructions"),
            Metric::Cycles => write!(f, "cycles"),
            Metric::PageFaults => write!(f, "page_faults"),
            Metric::ContextSwitches => write!(f, "context_switches"),
            Metric::CpuRunningMilliseconds(ref s) => write!(f, "{}_running_milliseconds", s),
        }
    }
}

pub struct Collector {
    perf: PerfCounters,
    sender: Sender<Metric>,
    clock: Clocksource,
}

impl Collector {
    pub fn new(listen: Option<String>) -> Collector {
        let mut config = Receiver::<Metric>::configure()
            .batch_size(1024)
            .duration(1)
            .capacity(4096)
            .poll_delay(Some(Duration::new(0, 1_000_000)))
            .service(true);

        if let Some(addr) = listen {
            config = config.http_listen(addr);
        }

        let mut receiver = config.build();

        let perf = PerfCounters::new();

        for metric in perf.get_metrics() {
            receiver.add_interest(Interest::Count(metric));
        }



        let sender = receiver.get_sender();
        let clocksource = receiver.get_clocksource();

        let schedstat = SchedstatCounters::new(clocksource.counter());
        for metric in schedstat.metrics() {
            receiver.add_interest(Interest::Count(metric));
        }

        thread::spawn(move || receiver.run());

        Collector {
            perf: perf,
            sender: sender,
            clock: clocksource,
        }
    }

    pub fn collect(&mut self) {
        let t0 = self.clock.counter();
        for counter in self.perf.counters.values_mut() {
            counter.start().expect("Could not start perf counter");
        }
        let mut schedstat = SchedstatCounters::new(t0);

        shuteye::sleep(Duration::new(0, 1_000_000));
        let t1 = self.clock.counter();
        for counter in self.perf.counters.values_mut() {
            counter.stop().expect("Could not start perf counter");
        }
        schedstat.sample(&mut self.sender, &self.clock);
        for (metric, counter) in &mut self.perf.counters {
            let _ = self.sender
                .send(Sample::counted(t0,
                                      t1,
                                      counter.read().expect("Could not read perf counter"),
                                      metric.clone()));
            counter.reset().expect("Could not reset perf counter");
        }
    }
}
