use metrics::Metric;
use perfcnt::PerfCounter;
use perfcnt::linux::{HardwareEventType, PerfCounterBuilderLinux, SoftwareEventType};
use std::collections::HashMap;
use x86;

pub struct PerfCounters {
    pub counters: HashMap<Metric, PerfCounter>,
}

impl PerfCounters {
    pub fn new() -> PerfCounters {
        let mut b = HashMap::new();
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
            }
            {
                b.insert(
                    metric,
                    PerfCounterBuilderLinux::from_intel_event_description(descr),
                );
            }
        }

        b.insert(
            Metric::Instructions,
            PerfCounterBuilderLinux::from_hardware_event(HardwareEventType::Instructions),
        );
        b.insert(
            Metric::Cycles,
            PerfCounterBuilderLinux::from_hardware_event(HardwareEventType::RefCPUCycles),
        );

        b.insert(
            Metric::PageFaults,
            PerfCounterBuilderLinux::from_software_event(SoftwareEventType::PageFaults),
        );
        b.insert(
            Metric::ContextSwitches,
            PerfCounterBuilderLinux::from_software_event(SoftwareEventType::ContextSwitches),
        );

        let mut counters = HashMap::new();
        for (metric, builder) in b {
            counters.insert(metric, builder.finish().unwrap());
        }
        PerfCounters { counters: counters }
    }

    pub fn get_metrics(&self) -> Vec<Metric> {
        let mut metrics = Vec::new();
        for metric in self.counters.keys() {
            let m = metric.clone();
            metrics.push(m);
        }
        metrics
    }
}
