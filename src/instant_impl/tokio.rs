use tokio::time::Instant;

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