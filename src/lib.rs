use std::{task::{Context, Poll}, time::{Duration, Instant}};

#[macro_use]
mod macros;

pub(crate) mod util;

mod interval;
mod sleep;
mod timeout;
mod delay_queue;

/// Copied from `tokio-util::time::delay_queue::wheel`
mod wheel;

mod delay_impl;

pub mod error;

// Re-exports
pub use interval::*;
pub use sleep::*;
pub use timeout::*;
pub use delay_queue::*;

pub trait Delay { }

pub trait AsyncDelay {
    type Value;

    fn delay(duration: Duration) -> Self;

    fn delay_until(deadline: Instant) -> Self;

    /// Some implementation do not expose the deadline, so this is an optional
    fn deadline(&self) -> Option<Instant>;

    fn poll_elapsed(&mut self, cx: &mut Context<'_>) -> Poll<Self::Value>;

    fn reset(&mut self, deadline: Instant);
}