use std::time::Instant as StdInstant;

impl crate::Instant for StdInstant {
    fn now() -> Self {
        StdInstant::now()
    }
}