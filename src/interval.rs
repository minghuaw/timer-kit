use std::{pin::Pin, time::{Duration, Instant}, task::{Context, Poll}, task::ready, future::poll_fn};

use crate::{AsyncDelay};

pub fn interval() {
    todo!()
}

pub struct Interval<D> {
    delay: Pin<Box<D>>,
    period: Duration,
}

impl<D> Interval<D>
where
    D: AsyncDelay + Unpin,
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
        let value = ready!(self.delay.poll_elapsed(cx));
        let next = Instant::now() + self.period;
        self.delay.reset(next);
        Poll::Ready(value)
    }

    pub async fn tick(&mut self) -> D::Value {
        poll_fn(|cx| self.poll_tick(cx)).await
    }

    pub fn reset(&mut self) {
        let deadline = Instant::now() + self.period;
        self.delay.reset(deadline);
    }
}