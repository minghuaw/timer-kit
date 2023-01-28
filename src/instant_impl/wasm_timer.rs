use wasm_timer::Instant;

impl crate::Instant for Instant {
    fn now() -> Self {
        wasm_timer::Instant::now()
    }
}