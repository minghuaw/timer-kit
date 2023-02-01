use std::io;

use fluvio_wasm_timer::{Delay, Instant};

impl crate::Delay for Delay {
    type Value = io::Result<()>;
    type Instant = Instant;

    fn delay(duration: std::time::Duration) -> Self {
        Delay::new(duration)
    }

    fn delay_until(deadline: Instant) -> Self {
        Delay::new_at(deadline)
    }

    fn deadline(&self) -> Option<Instant> {
        None
    }

    fn poll_elapsed(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Value> {
        std::future::Future::poll(self, cx)
    }

    fn reset(self: std::pin::Pin<&mut Self>, deadline: Instant) {
        self.get_mut().reset_at(deadline)
    }
}