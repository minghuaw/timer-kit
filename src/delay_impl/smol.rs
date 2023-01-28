use smol::Timer;

use std::time::Instant;

impl crate::Delay for Timer {
    type Value = Instant;

    type Instant = Instant;

    fn delay(duration: std::time::Duration) -> Self {
        Self::after(duration)
    }

    fn delay_until(deadline: Self::Instant) -> Self {
        Self::at(deadline)
    }

    fn deadline(&self) -> Option<Self::Instant> {
        None
    }

    fn poll_elapsed(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Value> {
        std::future::Future::poll(self, cx)
    }

    fn reset(self: std::pin::Pin<&mut Self>, deadline: Self::Instant) {
        let me = self.get_mut();
        *me = Self::at(deadline);
    }
}