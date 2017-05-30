extern crate x86;

use perfcnt::PerfCounter;
use perfcnt::linux::PerfCounterBuilderLinux;
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;
use tic::{Interest, Receiver};
//use x86;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Metric {
    MemoryLoadsL1Hit,
    MemoryLoadsL1Miss,
    MemoryLoadsL2Hit,
    MemoryLoadsL2Miss,
    MemoryLoadsL3Hit,
    MemoryLoadsL3Miss,
    MemoryLoadsTotal,
    MemoryLoadsSplit,
    MemoryStoresTotal,
    MemoryStoresSplit,
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Metric::MemoryLoadsL1Hit => write!(f, "telem_memory_loads_l1_hit"),
            Metric::MemoryLoadsL1Miss => write!(f, "telem_memory_loads_l1_miss"),
            Metric::MemoryLoadsL2Hit => write!(f, "telem_memory_loads_l2_hit"),
            Metric::MemoryLoadsL2Miss => write!(f, "telem_memory_loads_l2_miss"),
            Metric::MemoryLoadsL3Hit => write!(f, "telem_memory_loads_l3_hit"),
            Metric::MemoryLoadsL3Miss => write!(f, "telem_memory_loads_l3_miss"),
            Metric::MemoryLoadsTotal => write!(f, "telem_memory_loads_total"),
            Metric::MemoryStoresTotal => write!(f, "telem_memory_stores_total"),
            Metric::MemoryLoadsSplit => write!(f, "telem_memory_loads_split"),
            Metric::MemoryStoresSplit => write!(f, "telem_memory_stores_split"),
        }
    }
}

pub fn intel_core_counters() -> HashMap<Metric, PerfCounter> {
    let mut h = HashMap::new();
    for (key, descr) in x86::perfcnt::core_counters().unwrap() {
        if let Some(metric) = match *key {
               "MEM_LOAD_UOPS_RETIRED.L1_HIT" => Some(Metric::MemoryLoadsL1Hit),
               "MEM_LOAD_UOPS_RETIRED.L1_MISS" => Some(Metric::MemoryLoadsL1Miss),
               "MEM_LOAD_UOPS_RETIRED.L2_HIT" => Some(Metric::MemoryLoadsL2Hit),
               "MEM_LOAD_UOPS_RETIRED.L2_MISS" => Some(Metric::MemoryLoadsL2Miss),
               "MEM_LOAD_UOPS_RETIRED.L3_HIT" => Some(Metric::MemoryLoadsL3Hit),
               "MEM_LOAD_UOPS_RETIRED.L3_MISS" => Some(Metric::MemoryLoadsL3Miss),
               "MEM_UOPS_RETIRED.ALL_LOADS" => Some(Metric::MemoryLoadsTotal),
               "MEM_UOPS_RETIRED.ALL_STORES" => Some(Metric::MemoryStoresTotal),
               "MEM_UOPS_RETIRED.SPLIT_LOADS" => Some(Metric::MemoryLoadsSplit),
               "MEM_UOPS_RETIRED.SPLIT_STORES" => Some(Metric::MemoryStoresSplit),
               _ => None,
           } {
            h.insert(metric,
                     PerfCounterBuilderLinux::from_intel_event_description(&descr)
                         .finish()
                         .unwrap());
        }
    }
    h
}

pub fn init(listen: Option<String>) -> Receiver<Metric> {
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

    for (metric, _) in intel_core_counters() {
        receiver.add_interest(Interest::Count(metric));
    }

    receiver
}
