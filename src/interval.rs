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

#[cfg(test)]
#[cfg(any(
    feature = "tokio",
    feature = "smol",
    feature = "wasm-timer",
))]
mod test_util {
    use std::time::Duration;
    use futures_util::{poll, pin_mut, future::poll_fn};

    use crate::{Delay, Instant};

    macro_rules! assert_interval_poll_ready {
        ($interval:ident) => {
            let fut = poll_fn(|cx| $interval.poll_tick(cx));
            pin_mut!(fut);
            assert!(poll!(fut).is_ready());
        };
    }

    macro_rules! assert_interval_poll_pending {
        ($interval:ident) => {
            let fut = poll_fn(|cx| $interval.poll_tick(cx));
            pin_mut!(fut);
            assert!(poll!(fut).is_pending());
        };
    }

    pub(super) async fn interval_zero_duration<D>() 
    where
        D: Delay,
        D::Instant: Unpin,
    {
        let _ = crate::interval::<D>(Duration::from_millis(0));
    }

    pub(super) async fn burst<D>() 
    where
        D: Delay,
        D::Instant: Unpin,
    {
        let start = D::Instant::now() + Duration::from_millis(100);
        let mut interval = crate::interval_at::<D>(start, Duration::from_millis(300));
        interval.set_missed_tick_behavior(crate::MissedTickBehavior::Burst);

        // Give a 1ms buffer
        crate::sleep::<D>(Duration::from_millis(1)).await;
        assert_interval_poll_pending!(interval);

        // Interval starts at 100ms
        crate::sleep::<D>(Duration::from_millis(100)).await;
        assert_interval_poll_ready!(interval);

        // Miss two tick
        crate::sleep::<D>(Duration::from_millis(700)).await;
        assert_interval_poll_ready!(interval);
        assert_interval_poll_ready!(interval);
        assert_interval_poll_pending!(interval);

        crate::sleep::<D>(Duration::from_millis(200)).await;
        assert_interval_poll_ready!(interval);
        assert_interval_poll_pending!(interval);
    }

    pub(super) async fn delay<D>() 
    where
        D: Delay,
        D::Instant: Unpin,
    {
        let start = D::Instant::now() + Duration::from_millis(100);
        let mut interval = crate::interval_at::<D>(start, Duration::from_millis(300));
        interval.set_missed_tick_behavior(crate::MissedTickBehavior::Delay);

        // Give a 1ms buffer
        crate::sleep::<D>(Duration::from_millis(1)).await;
        assert_interval_poll_pending!(interval);

        // Interval starts at 100ms
        crate::sleep::<D>(Duration::from_millis(100)).await;
        assert_interval_poll_ready!(interval);

        // Miss two tick
        crate::sleep::<D>(Duration::from_millis(700)).await;
        assert_interval_poll_ready!(interval);
        assert_interval_poll_pending!(interval);

        // Next tick is delayed until 1100ms
        crate::sleep::<D>(Duration::from_millis(200)).await;
        assert_interval_poll_pending!(interval);
        crate::sleep::<D>(Duration::from_millis(100)).await;
        assert_interval_poll_ready!(interval);

        // Next tick is delayed until 1400ms
        crate::sleep::<D>(Duration::from_millis(100)).await;
        assert_interval_poll_pending!(interval);
        crate::sleep::<D>(Duration::from_millis(200)).await;
        assert_interval_poll_ready!(interval);
    }

    pub(super) async fn skip<D>() 
    where
        D: Delay,
        D::Instant: Unpin,
    {
        let start = D::Instant::now() + Duration::from_millis(100);
        let mut interval = crate::interval_at::<D>(start, Duration::from_millis(300));
        interval.set_missed_tick_behavior(crate::MissedTickBehavior::Skip);

        // Give a 1ms buffer
        crate::sleep::<D>(Duration::from_millis(1)).await;
        assert_interval_poll_pending!(interval);

        // Interval starts at 100ms
        crate::sleep::<D>(Duration::from_millis(100)).await;
        assert_interval_poll_ready!(interval);

        // Miss two tick
        crate::sleep::<D>(Duration::from_millis(700)).await;
        assert_interval_poll_ready!(interval);
        assert_interval_poll_pending!(interval);
        
        crate::sleep::<D>(Duration::from_millis(200)).await;
        assert_interval_poll_ready!(interval);
        assert_interval_poll_pending!(interval);

        crate::sleep::<D>(Duration::from_millis(300)).await;
        assert_interval_poll_ready!(interval);
    }

    pub(super) async fn reset<D>() 
    where
        D: Delay,
        D::Instant: Unpin,
    {
        let start = D::Instant::now() + Duration::from_millis(100);
        let mut interval = crate::interval_at::<D>(start, Duration::from_millis(300));

        // Give a 1ms buffer
        crate::sleep::<D>(Duration::from_millis(1)).await;
        assert_interval_poll_pending!(interval);

        // Interval starts at 100ms
        crate::sleep::<D>(Duration::from_millis(100)).await;
        assert_interval_poll_ready!(interval);

        crate::sleep::<D>(Duration::from_millis(300)).await;
        assert_interval_poll_ready!(interval);

        crate::sleep::<D>(Duration::from_millis(50)).await;
        interval.reset();

        crate::sleep::<D>(Duration::from_millis(250)).await;
        assert_interval_poll_pending!(interval);

        crate::sleep::<D>(Duration::from_millis(50)).await;
        assert_interval_poll_ready!(interval);

        crate::sleep::<D>(Duration::from_millis(300)).await;
        assert_interval_poll_ready!(interval);
    }
}

cfg_tokio! {
    #[cfg(test)]
    mod tokio_tests {
        use super::test_util;

        #[tokio::test]
        #[should_panic]
        async fn interval_zero_duration() {
            test_util::interval_zero_duration::<tokio::time::Sleep>().await;
        }

        #[tokio::test]
        async fn burst() {
            test_util::burst::<tokio::time::Sleep>().await;
        }

        #[tokio::test]
        async fn delay() {
            test_util::delay::<tokio::time::Sleep>().await;
        }

        #[tokio::test]
        async fn skip() {
            test_util::skip::<tokio::time::Sleep>().await;
        }

        #[tokio::test]
        async fn reset() {
            test_util::reset::<tokio::time::Sleep>().await;
        }
    }
}

cfg_smol! {
    #[cfg(test)]
    mod smol_test {
        use super::test_util;

        #[smol_potat::test]
        #[should_panic]
        async fn interval_zero_duration() {
            test_util::interval_zero_duration::<smol::Timer>().await;
        }
    
        #[smol_potat::test]
        async fn burst() {
            test_util::burst::<smol::Timer>().await;
        }
    
        #[smol_potat::test]
        async fn delay() {
            test_util::delay::<smol::Timer>().await;
        }
    
        #[smol_potat::test]
        async fn skip() {
            test_util::skip::<smol::Timer>().await;
        }
    
        #[smol_potat::test]
        async fn reset() {
            test_util::reset::<smol::Timer>().await;
        }
    }
}