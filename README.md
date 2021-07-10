# ECHONET Lite for Rust

This crate is pure Rust ECHONET Lite implementation including
- serde implementation of ECHONET Lite packet
- detailed property configurations of ECHONET Device objects (WIP)

but not included
- transport layer (usually, UDP with IPv4/IPv6) implementation
- specific ECHONET Lite object behavior

see exmaples to know how to communicate with ECHONTE Lite node.

## About ECHONET Lite

`ECHONET Lite` provides an infrastructure of **Smart house** in order to solve the global environmental issues.
This technology defines a common interface for home appliances such as air conditioners, smart meters, and house hold solar power system.

https://echonet.jp/english/

The specification is open, see the below web page for detailed ECHONET Lite specifications.

https://echonet.jp/spec-en/#standard-01

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
