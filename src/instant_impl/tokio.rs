use tokio::time::Instant;

impl crate::Instant for Instant {
    fn now() -> Self {
        Instant::now()
    }
}