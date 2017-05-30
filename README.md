# telem - telemetry agent
a Prometheus-compatible telemetry agent which exposes Intel Performance counters

## Usage

Note: telem may require sudo for reading the performance counters

```shell
cargo build --release && sudo ./target/release/telem --listen 0.0.0.0:42024
```

## Features

* Prometheus-compatible
* Intel Performance counters

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
