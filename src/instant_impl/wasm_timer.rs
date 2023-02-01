use wasm_timer::Instant;

impl crate::Instant for Instant {
    fn now() -> Self {
        Instant::now()
    }
}