use perfcnt::PerfCounter;
use perfcnt::linux::{HardwareEventType, PerfCounterBuilderLinux, SoftwareEventType};
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;
use tic::{Interest, Receiver};
use x86;

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
        }
    }
}

pub fn get_counters() -> HashMap<Metric, PerfCounter> {
    let mut h = HashMap::new();
    for (key, descr) in x86::perfcnt::core_counters().unwrap() {
        if let Some(metric) = match *key {
               "MEM_LOAD_UOPS_RETIRED.L1_HIT" => Some(Metric::MemoryLoadsL1Hit),
               "MEM_LOAD_UOPS_RETIRED.L1_MISS" => Some(Metric::MemoryLoadsL1Miss),
               "MEM_LOAD_UOPS_RETIRED.L2_HIT" => Some(Metric::MemoryLoadsL2Hit),
               "MEM_LOAD_UOPS_RETIRED.L2_MISS" => Some(Metric::MemoryLoadsL2Miss),
               "MEM_LOAD_UOPS_RETIRED.L3_HIT" => Some(Metric::MemoryLoadsL3Hit),
               "MEM_LOAD_UOPS_RETIRED.L3_MISS" => Some(Metric::MemoryLoadsL3Miss),
               "MEM_UOPS_RETIRED.ALL_LOADS" => Some(Metric::MemoryLoads),
               "MEM_UOPS_RETIRED.ALL_STORES" => Some(Metric::MemoryStores),
               "MEM_UOPS_RETIRED.SPLIT_LOADS" => Some(Metric::MemoryLoadsSplit),
               "MEM_UOPS_RETIRED.SPLIT_STORES" => Some(Metric::MemoryStoresSplit),
               "UOPS_RETIRED.ALL" => Some(Metric::UopsRetired),
               _ => None,
           } {
            h.insert(metric,
                     PerfCounterBuilderLinux::from_intel_event_description(descr)
                         .finish()
                         .unwrap());
        }
    }

    h.insert(Metric::Instructions,
             PerfCounterBuilderLinux::from_hardware_event(HardwareEventType::Instructions)
                 .finish()
                 .unwrap());
    h.insert(Metric::Cycles,
             PerfCounterBuilderLinux::from_hardware_event(HardwareEventType::RefCPUCycles)
                 .finish()
                 .unwrap());

    h.insert(Metric::PageFaults,
             PerfCounterBuilderLinux::from_software_event(SoftwareEventType::PageFaults)
                 .finish()
                 .unwrap());
    h.insert(Metric::ContextSwitches,
             PerfCounterBuilderLinux::from_software_event(SoftwareEventType::ContextSwitches)
                 .finish()
                 .unwrap());

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

    for (metric, _) in get_counters() {
        receiver.add_interest(Interest::Count(metric));
    }

    receiver
}
