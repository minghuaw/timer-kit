use std::{pin::Pin, time::{Duration, Instant}, future::Future};

use crate::{AsyncDelay};

pub fn sleep() {
    todo!()
}

pub fn sleep_until() {
    todo!()
}

#[derive(Debug)]
pub struct Sleep<D> {
    delay: D,
    deadline: Instant,
}

impl<D> Sleep<D>
where
    D: AsyncDelay + Unpin,
{
    pub fn new(duration: Duration) -> Self {
        let delay = D::delay(duration);
        let deadline = delay.deadline().unwrap_or(Instant::now() + duration);
        Self {
            delay,
            deadline,
        }
    }

    pub fn new_until(deadline: Instant) -> Self {
        Self {
            delay: D::delay_until(deadline),
            deadline,
        }
    }

    pub fn reset(&mut self, deadline: Instant) {
        self.delay.reset(deadline);
    }

    pub fn deadline(&self) -> Instant {
        self.deadline
    }
}

impl<D> Future for Sleep<D>
where
    D: AsyncDelay + Unpin,
{
    type Output = D::Value;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.get_mut().delay.poll_elapsed(cx)
    }
}
