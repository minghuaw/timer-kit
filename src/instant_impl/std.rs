use std::time::Instant as StdInstant;

impl crate::Instant for StdInstant {
    fn now() -> Self {
        StdInstant::now()
    }

    fn checked_add(&self, duration: std::time::Duration) -> Option<Self> {
        StdInstant::checked_add(&self, duration)
    }

    fn checked_sub(&self, duration: std::time::Duration) -> Option<Self> {
        StdInstant::checked_sub(&self, duration)
    }

    fn checked_duration_since(&self, other: Self) -> Option<std::time::Duration> {
        StdInstant::checked_duration_since(&self, other)
    }

    fn duration_since(&self, other: Self) -> std::time::Duration {
        StdInstant::duration_since(&self, other)
    }
}