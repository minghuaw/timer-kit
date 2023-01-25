use std::pin::Pin;

pub struct DelayQueue<T> {
    delay: Pin<Box<T>>,
}