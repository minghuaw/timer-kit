use std::pin::Pin;

pub struct Timeout<T> {
    delay: Pin<Box<T>>
}