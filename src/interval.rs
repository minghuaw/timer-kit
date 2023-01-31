use std::{pin::Pin, time::{Duration}, task::{Context, Poll}, task::ready, future::poll_fn};

use futures_util::{Stream, Future};

use crate::{Delay, Sleep};

const BUFFER_TIMEOUT: Duration = Duration::from_millis(5);

pub fn interval<D>(duration: Duration) -> Interval<D> 
where
    D: Delay,
    D::Instant: Unpin,
{
    Interval::new(duration)
}

pub fn interval_at<D>(start: D::Instant, duration: Duration) -> Interval<D> 
where
    D: Delay,
    D::Instant: Unpin,
{
    Interval::new_at(start, duration)
}

/// Copied from `tokio::time::interval`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissedTickBehavior {
    Burst,
    Delay,
    Skip,
}

impl MissedTickBehavior {
    fn next_timeout<D>(&self, timeout: D::Instant, now: D::Instant, period: Duration) -> D::Instant 
    where
        D: Delay,
    {
        match self {
            Self::Burst => timeout + period,
            Self::Delay => now + period,
            Self::Skip => {
                now + period
                    - Duration::from_nanos(
                        ((now - timeout).as_nanos() % period.as_nanos())
                            .try_into()
                            // This operation is practically guaranteed not to
                            // fail, as in order for it to fail, `period` would
                            // have to be longer than `now - timeout`, and both
                            // would have to be longer than 584 years.
                            //
                            // If it did fail, there's not a good way to pass
                            // the error along to the user, so we just panic.
                            .expect(
                                "too much time has elapsed since the interval was supposed to tick",
                            ),
                    )
            }
        }
    }
}

impl Default for MissedTickBehavior {
    fn default() -> Self {
        Self::Burst
    }
}

pub struct Interval<D: Delay> {
    delay: Pin<Box<Sleep<D>>>,
    missed_tick_behavior: MissedTickBehavior,
    period: Duration,
}

impl<D> Interval<D>
where
    D: Delay,
    D::Instant: Unpin,
{
    pub fn new(period: Duration) -> Self {
        assert!(period > Duration::new(0, 0), "period must be non-zero");
        Self {
            delay: Box::pin(Sleep::new(period)),
            missed_tick_behavior: MissedTickBehavior::default(),
            period,
        }
    }

    pub fn new_at(start: D::Instant, period: Duration) -> Self {
        assert!(period > Duration::new(0, 0), "period must be non-zero");
        Self {
            delay: Box::pin(Sleep::new_until(start)),
            missed_tick_behavior: MissedTickBehavior::default(),
            period,
        }
    }

    pub fn missed_tick_behavior(&self) -> MissedTickBehavior {
        self.missed_tick_behavior
    }

    pub fn set_missed_tick_behavior(&mut self, behavior: MissedTickBehavior) {
        self.missed_tick_behavior = behavior;
    }

    pub fn period(&self) -> Duration {
        self.period
    }

    pub fn poll_tick(&mut self, cx: &mut Context<'_>) -> Poll<D::Value> {
        use crate::Instant;

        let value = ready!(self.delay.as_mut().poll(cx));

        let timeout = self.delay.deadline();

        let now = D::Instant::now();

        let next = if now > timeout + BUFFER_TIMEOUT {
            self.missed_tick_behavior.next_timeout::<D>(timeout, now, self.period)
        } else {
            timeout + self.period
        };

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

impl<D> Stream for Interval<D>
where
    D: Delay,
    D::Instant: Unpin,
{
    type Item = D::Value;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let value = ready!(self.get_mut().poll_tick(cx));
        Poll::Ready(Some(value))
    }
} 
