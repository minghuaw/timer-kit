use std::io;

impl crate::Delay for wasm_timer::Delay {
    type Value = io::Result<()>;

    fn delay(duration: std::time::Duration) -> Self {
        wasm_timer::Delay::new(duration)
    }

    fn delay_until(deadline: std::time::Instant) -> Self {
        wasm_timer::Delay::new_at(deadline)
    }

    fn deadline(&self) -> Option<std::time::Instant> {
        todo!()
    }

    fn poll_elapsed(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Value> {
        todo!()
    }

    fn reset(self: std::pin::Pin<&mut Self>, deadline: std::time::Instant) {
        todo!()
    }
}