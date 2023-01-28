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
pub trait Delay {
    type Value;
    type Instant: Instant;

    fn delay(duration: Duration) -> Self;

    fn delay_until(deadline: Self::Instant) -> Self;

    /// Some implementation do not expose the deadline, so this is an optional
    fn deadline(&self) -> Option<Self::Instant>;

    fn poll_elapsed(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Value>;

    fn reset(self: Pin<&mut Self>, deadline: Self::Instant);
}

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

    fn duration_since(&self, other: Self) -> std::time::Duration;

    fn checked_add(&self, duration: Duration) -> Option<Self>;

    fn checked_sub(&self, duration: Duration) -> Option<Self>;

    fn checked_duration_since(&self, other: Self) -> Option<Duration>;
}
