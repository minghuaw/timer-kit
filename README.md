# timer-kit

A timer toolkit that is generic over the underlying timer implementation.

This crate does not implement any platform-specific timer but uses a generic abstraction
over the timer implementation to provide a set of timer related tools:

1. [`sleep`]/[`Sleep`]
2. [`timeout`]/[`Timeout`]
3. [`interval`]/[`Interval`]
4. [`DelayQueue`]

The core of this crate is the [`Delay`] trait, and it is implemented for the following types
buy enabling the corresponding features:

| Type | Feature | Target Arch |
| ---- | ------- | ----------- |
| [`tokio::time::Sleep`] | `"tokio"` | non-wasm32 |
| [`smol::Timer`] | `"smol"` | non-wasm32 |
| [`futures_timer::Delay`] | `"futures-timer"` | non-wasm32 |
| [`wasm_timer::Delay`] | `"wasm-timer"` | wasm32 |
| [`fluvio_wasm_timer::Delay`] | `"fluvio-wasm-timer"` | wasm32 |

## Examples

The usage remains mostly similar to those provided in `tokio::time` with one additional generic
type parameter `D` which is the type of the underlying timer implementation. Please refer to the
documentation of the corresponding types for more details.
