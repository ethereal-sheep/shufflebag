# shufflebag

[![Crates.io](https://img.shields.io/crates/v/shufflebag.svg)](https://crates.io/crates/shufflebag)
[![Docs.rs](https://docs.rs/shufflebag/badge.svg)](https://docs.rs/shufflebag)
[![CI](https://github.com/ethereal-sheep/shufflebag/workflows/CI/badge.svg)](https://github.com/ethereal-sheep/shufflebag/actions)
[![Coverage Status](https://coveralls.io/repos/github/ethereal-sheep/shufflebag/badge.svg?branch=main)](https://coveralls.io/github/ethereal-sheep/shufflebag?branch=main)

<!-- cargo-rdme start -->

A shuffle bag implementation in `rust`.

The bag allows for a pseudo random drawing of its elements
by assigning a random float64 value to each value and storing
it in a maximal `BinaryHeap<T>`.

Therefore, `push` and `pop` complexities follow those of the
`std::collections::BinaryHeap`.

<!-- cargo-rdme end -->

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install shufflebag`

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
