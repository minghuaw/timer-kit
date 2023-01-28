use std::{pin::Pin, future::Future, time::{Duration}, task::{Poll}};

use pin_project_lite::pin_project;

use crate::{Delay, error::{Elapsed}};

// pub fn timeout() {
//     todo!()
// }

// pub fn timeout_at() {
//     todo!()
// }

pin_project! {
    pub struct Timeout<D, Fut> {
        #[pin]
        delay: D,

        #[pin]
        future: Fut,
    }
}

impl<D, Fut> Timeout<D, Fut>
where
    D: Delay + Unpin,
    Fut: Future,
{
    pub fn new(duration: Duration, future: Fut) -> Self {
        Self {
            delay: D::delay(duration),
            future,
        }
    }

    pub fn new_at(deadline: D::Instant, future: Fut) -> Self {
        Self {
            delay: D::delay_until(deadline),
            future,
        }
    }
}

impl<D, Fut> Future for Timeout<D, Fut> 
where
    D: Delay + Unpin,
    Fut: Future,
{
    type Output = Result<Fut::Output, Elapsed>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let mut this = self.project();

        if let Poll::Ready(output) = this.future.poll(cx) {
            return Poll::Ready(Ok(output));
        }

        match this.delay.as_mut().poll_elapsed(cx) {
            Poll::Ready(_) => Poll::Ready(Err(Elapsed::new())),
            Poll::Pending => Poll::Pending,
        }
    }
}