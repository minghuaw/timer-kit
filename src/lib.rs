//! A timer toolkit that is generic over the underlying timer implementation. 
//! 
//! This crate does not implement any platform-specific timer but uses a generic abstraction
//! over the timer implementation to provide a set of timer related tools:
//! 
//! 1. [`sleep`]/[`Sleep`]
//! 2. [`timeout`]/[`Timeout`]
//! 3. [`interval`]/[`Interval`]
//! 4. [`DelayQueue`]
//! 
//! The core of this crate is the [`Delay`] trait, and it is implemented for the following types
//! buy enabling the corresponding features:
//! 
//! | Type | Feature | Target Arch |
//! | ---- | ------- | ----------- |
//! | [`tokio::time::Sleep`] | `"tokio"` | non-wasm32 |
//! | [`smol::Timer`] | `"smol"` | non-wasm32 |
//! | [`futures_timer::Delay`] | `"futures-timer"` | non-wasm32 |
//! | [`wasm_timer::Delay`] | `"wasm-timer"` | wasm32 |
//! | [`fluvio_wasm_timer::Delay`] | `"fluvio-wasm-timer"` | wasm32 |
//!
//! # Examples
//! 
//! The usage remains mostly similar to those provided in `tokio::time` with one additional generic
//! type parameter `D` which is the type of the underlying timer implementation. Please refer to the
//! documentation of the corresponding types for more details.

use std::{
    ops::{Add, Sub},
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

#[macro_use]
mod macros;

pub(crate) mod util;

mod delay_queue;
mod interval;
mod sleep;
mod timeout;

/// Copied from `tokio-util::time::delay_queue::wheel`
mod wheel;

mod delay_impl;
mod instant_impl;

pub mod error;

// Re-exports
pub use delay_queue::*;
pub use interval::*;
pub use sleep::*;
pub use timeout::*;

/// A trait that defines a delay, which is the fundamental building block of this crate.
/// 
/// # Implementations
/// 
/// Implementations for the following types are provided with the corresponding features enabled:
/// 
/// | Type | Feature | Target Arch |
/// | ---- | ------- | ----------- |
/// | [`tokio::time::Sleep`] | `"tokio"` | non-wasm32 |
/// | [`smol::Timer`] | `"smol"` | non-wasm32 |
/// | [`futures_timer::Delay`] | `"futures-timer"` | non-wasm32 |
/// | [`wasm_timer::Delay`] | `"wasm-timer"` | wasm32 |
/// | [`fluvio_wasm_timer::Delay`] | `"fluvio-wasm-timer"` | wasm32 |
/// 
/// User could also provide their own implementations for other types to use the timer
/// functionalities provided by this crate.
pub trait Delay {
    /// The type of value returned by the delay upon completion of `poll_elapsed`.
    type Value;

    /// The type of instant used by the delay.
    type Instant: Instant;

    fn delay(duration: Duration) -> Self;

    fn delay_until(deadline: Self::Instant) -> Self;

    /// Some implementation do not expose the deadline, so this is an optional
    fn deadline(&self) -> Option<Self::Instant>;

    fn poll_elapsed(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Value>;

    fn reset(self: Pin<&mut Self>, deadline: Self::Instant);
}

/// A trait that defines an instant.
/// 
/// # Implementations
/// 
/// Implementations for the following types are provided with the corresponding features enabled:
/// 
/// | Type | Feature | Target Arch |
/// | ---- | ------- | ----------- |
/// | [`std::time::Instant`] | `"std"` | non-wasm32 |
/// | [`tokio::time::Instant`] | `"tokio"` | non-wasm32 |
/// | [`wasm_timer::Instant`] | `"wasm-timer"` | wasm32 |
/// | [`fluvio_wasm_timer::Instant`] | `"fluvio-wasm-timer"` | wasm32 |
/// 
/// User could also provide their own implementations for other types to use the timer
/// functionalities provided by this crate.
pub trait Instant
where
    Self: Add<Duration, Output = Self>
        + Sub<Duration, Output = Self>
        + Sub<Self, Output = Duration>
        + Sized
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord,
{
    fn now() -> Self;
}
