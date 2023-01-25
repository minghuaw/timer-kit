use std::pin::Pin;

pub struct Sleep<T> {
    delay: Pin<Box<T>>
}