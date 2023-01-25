use std::pin::Pin;

pub struct Interval<T> {
    delay: Pin<Box<T>>,
}