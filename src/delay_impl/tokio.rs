use std::{pin::Pin};

use futures_util::Future;

use tokio::time::{Sleep, Instant};

impl crate::Delay for Sleep {
    type Value = ();
    type Instant = Instant;

    fn delay(duration: std::time::Duration) -> Self {
        tokio::time::sleep(duration)
    }

    fn delay_until(deadline: Instant) -> Self {
        tokio::time::sleep_until(deadline)
    }

    fn deadline(&self) -> Option<Instant> {
        Some(self.deadline())
    }

    fn poll_elapsed(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Value> {
        self.poll(cx)
    }

    fn reset(self: Pin<&mut Self>, deadline: Instant) {
        tokio::time::Sleep::reset(self, deadline)
    }
}

impl crate::Instant for Instant {
    fn now() -> Self {
        Instant::now()
    }

    fn duration_since(&self, other: Self) -> std::time::Duration {
        self.duration_since(other)
    }

    fn checked_add(&self, duration: std::time::Duration) -> Option<Self> {
        self.checked_add(duration)
    }

    fn checked_sub(&self, duration: std::time::Duration) -> Option<Self> {
        self.checked_sub(duration)
    }

    fn checked_duration_since(&self, other: Self) -> Option<std::time::Duration> {
        self.checked_duration_since(other)
    }
}