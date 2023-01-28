use std::{pin::Pin};

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
        std::future::Future::poll(self, cx)
    }

    fn reset(self: Pin<&mut Self>, deadline: Instant) {
        tokio::time::Sleep::reset(self, deadline)
    }
}
