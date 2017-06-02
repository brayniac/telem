use metrics::Metric;
use std::collections::HashMap;
use tic::{Clocksource, Sample, Sender};
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

#[derive(Clone)]
pub struct Counters {
	time: u64,
	data: HashMap<Metric, u64>,
}

impl Counters {
	pub fn new(time: u64) -> Counters {
		let mut data = HashMap::new();

		let f = File::open("/proc/schedstat").unwrap();
		let f = BufReader::new(f);

		for line in f.lines() {
			let line = line.unwrap();
			let tokens: Vec<&str> = line.split_whitespace().collect();
			if tokens.len() != 10 {
				continue;
			}
			let label = tokens[0];
			let value = tokens[7];
			data.insert(Metric::CpuRunningMilliseconds(label.to_owned()), value.parse().unwrap_or(0));

		    //println!("{}", line.unwrap());
		}

		Counters {
			time: time,
			data: data,
		}
	}

	pub fn metrics(&self) -> Vec<Metric> {
		let mut v = Vec::new();
		for metric in self.data.keys() {
			v.push(metric.clone());
		}
		v
	}

	pub fn sample(&mut self, sender: &mut Sender<Metric>, clock: &Clocksource) {
		let new = Counters::new(clock.counter());
		for (metric, v1) in new.clone().data {
			if let Some(v0) = self.data.get(&metric) {
				let _ = sender.send(Sample::counted(self.time, new.time, v1 - v0, metric));
			}
		}
		self.data = new.data;
		self.time = new.time;
	}
}