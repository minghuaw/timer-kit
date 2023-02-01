use futures_timer::Delay;

use std::time::Instant;

impl crate::Delay for Delay {
    type Value = ();

    type Instant = Instant;

    fn delay(duration: std::time::Duration) -> Self {
        Delay::new(duration)
    }

    fn delay_until(deadline: Self::Instant) -> Self {
        let now = Instant::now();
        let duration = deadline.duration_since(now);
        Delay::new(duration)
    }

    fn deadline(&self) -> Option<Self::Instant> {
        None
    }

    fn poll_elapsed(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Value> {
        std::future::Future::poll(self, cx)
    }

    fn reset(self: std::pin::Pin<&mut Self>, deadline: Self::Instant) {
        let now = Instant::now();
        let duration = deadline.duration_since(now);
        futures_timer::Delay::reset(self.get_mut(), duration)
    }
}