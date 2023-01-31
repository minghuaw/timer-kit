use std::{pin::Pin, time::{Duration}, future::Future};

use crate::{Delay, Instant};

pub fn sleep<D>(duration: Duration) -> Sleep<D> 
where
    D: Delay,
{
    Sleep::new(duration)
}

pub fn sleep_until<D>(deadline: D::Instant) -> Sleep<D> 
where
    D: Delay,
{
    Sleep::new_until(deadline)
}

#[derive(Debug)]
pub struct Sleep<D: Delay> {
    delay: Pin<Box<D>>,
    deadline: D::Instant,
}

impl<D> Sleep<D>
where
    D: Delay,
{
    pub(crate) fn new(duration: Duration) -> Self 
    {
        let delay = Box::pin(D::delay(duration));
        let deadline = delay.deadline().unwrap_or(D::Instant::now() + duration);
        Self {
            delay,
            deadline,
        }
    }

    pub(crate) fn new_until(deadline: D::Instant) -> Self {
        Self {
            delay: Box::pin(D::delay_until(deadline)),
            deadline,
        }
    }

    pub fn reset(&mut self, deadline: D::Instant) {
        self.deadline = deadline;
        self.delay.as_mut().reset(deadline);
    }

    pub fn deadline(&self) -> D::Instant {
        self.deadline
    }
}

impl<D> Future for Sleep<D>
where
    D: Delay,
    D::Instant: Unpin,
{
    type Output = D::Value;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.get_mut().delay.as_mut().poll_elapsed(cx)
    }
}
