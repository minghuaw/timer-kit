# timer-kit

A timer toolkit that is generic over the underlying timer implementation.

[![crate_version](https://img.shields.io/crates/v/timer-kit.svg?style=flat)](https://crates.io/crates/timer-kit)
[![docs_version](https://img.shields.io/badge/docs-latest-blue.svg?style=flat)](https://docs.rs/timer-kit/latest/timer_kit/)

This crate does not implement any platform-specific timer but uses a generic abstraction over
the timer implementation to provide a set of timer related tools:

1. [`sleep()`]/[`Sleep`]
2. [`timeout()`]/[`Timeout`]
3. [`interval()`]/[`Interval`]
4. [`DelayQueue`]

This crate currently does not provide any feature beyond the ones that is already provided by
`tokio`, so this crate is completely not needed if you are already using `tokio` in your
project.

The core of this crate is the [`Delay`] trait, and it is implemented for the following types by
enabling the corresponding features:

| Type | Feature | Target Arch |
| ---- | ------- | ----------- |
| [`tokio::time::Sleep`] | `"tokio"` | non-wasm32 |
| [`smol::Timer`] | `"smol"` | non-wasm32 |
| [`futures_timer::Delay`] | `"futures-timer"` | non-wasm32 |
| [`wasm_timer::Delay`] | `"wasm-timer"` | wasm32 |
| [`fluvio_wasm_timer::Delay`] | `"fluvio-wasm-timer"` | wasm32 |

## WebAssembly support

Support for `wasm32-unknown-unknown` target depends on the chosen timer implementation.
`wasm-timer` and `fluvio-wasm-timer` are the only two wasm timer implementations that are
currently supported.

## Examples

The usage remains mostly similar to those provided in `tokio::time` with one additional generic
type parameter `D` which is the type of the underlying timer implementation. Please refer to the
documentation of the corresponding types for more details.

License: MIT/Apache-2.0
