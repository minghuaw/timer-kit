use std::{pin::Pin, time::{Duration}, task::{Context, Poll}, task::ready, future::poll_fn};

use crate::{Delay};

// pub fn interval() {
//     todo!()
// }

pub struct Interval<D> {
    delay: Pin<Box<D>>,
    period: Duration,
}

impl<D> Interval<D>
where
    D: Delay + Unpin,
{
    pub fn new(period: Duration) -> Self {
        Self {
            delay: Box::pin(D::delay(period)),
            period,
        }
    }

    pub fn period(&self) -> Duration {
        self.period
    }

    pub fn poll_tick(&mut self, cx: &mut Context<'_>) -> Poll<D::Value> {
        use crate::Instant;

        let value = ready!(self.delay.as_mut().poll_elapsed(cx));
        let next = D::Instant::now() + self.period;
        self.delay.as_mut().reset(next);
        Poll::Ready(value)
    }

    pub async fn tick(&mut self) -> D::Value {
        poll_fn(|cx| self.poll_tick(cx)).await
    }

    pub fn reset(&mut self) {
        use crate::Instant;

        let deadline = D::Instant::now() + self.period;
        self.delay.as_mut().reset(deadline);
    }
}