use std::{pin::Pin, time::{Duration}, task::{Context, Poll}, task::ready, future::poll_fn};

use futures_util::{Stream, Future};

use crate::{Delay, Sleep};

const BUFFER_TIMEOUT: Duration = Duration::from_millis(5);

/// Creates new [`Interval`] that yields with interval of period. The first tick completes
/// immediately. The default [`MissedTickBehavior`] is [`MissedTickBehavior::Burst`], but this
/// can be configured by calling [`Interval::set_missed_tick_behavior`].
/// 
/// # Panics
/// 
/// This function will panic if `period` is zero.
/// 
/// # Example
/// 
/// Creates an interval with smol's timer
/// 
/// ```rust,no_run
/// use std::time::Duration;
/// use timer_kit::interval;
/// 
/// let mut interval = interval::<smol::Timer>(Duration::from_millis(100));
/// 
/// interval.tick().await;
/// interval.tick().await;
/// interval.tick().await;
/// ```
/// 
/// Creates an interval with `fluvio_wasm_timer::Delay`
/// 
/// ```rust,no_run
/// use std::time::Duration;
/// use timer_kit::interval;
/// 
/// let mut interval = interval::<fluvio_wasm_timer::Delay>(Duration::from_millis(100));
/// 
/// interval.tick().await;
/// interval.tick().await;
/// interval.tick().await;
/// ```
pub fn interval<D>(duration: Duration) -> Interval<D> 
where
    D: Delay,
    D::Instant: Unpin,
{
    Interval::new(duration)
}

/// Creates new [`Interval`] that yields with interval of period with the first tick completing
/// at start. The default [`MissedTickBehavior`] is [`MissedTickBehavior::Burst`], but this can
/// be configured by calling [`Interval::set_missed_tick_behavior`].
/// 
/// # Panics
/// 
/// This function will panic if `period` is zero.
/// 
/// # Example
/// 
/// Creates an interval with smol's timer
/// 
/// ```rust,no_run
/// use std::time::{Duration, Instant};
/// use timer_kit::interval_at;
/// 
/// let mut interval = interval_at::<smol::Timer>(Instant::now(), Duration::from_millis(100));
/// 
/// interval.tick().await;
/// interval.tick().await;
/// interval.tick().await;
/// ```
/// 
/// Creates an interval with `fluvio_wasm_timer::Delay`
/// 
/// ```rust,no_run
/// use std::time::{Duration};
/// use fluvio_wasm_timer::Instant;
/// use timer_kit::interval_at;
/// 
/// let mut interval = interval_at::<fluvio_wasm_timer::Delay>(Instant::now(), Duration::from_millis(100));
/// 
/// interval.tick().await;
/// interval.tick().await;
/// interval.tick().await;
/// ```
pub fn interval_at<D>(start: D::Instant, duration: Duration) -> Interval<D> 
where
    D: Delay,
    D::Instant: Unpin,
{
    Interval::new_at(start, duration)
}

/// Ported from [`tokio::time::interval::MissedTickBehavior`]
/// 
/// # Default
/// 
/// The default behavior is [`MissedTickBehavior::Burst`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissedTickBehavior {
    /// Ticks as fast as possible until caught up.
    Burst,

    /// Tick at multiples of `period` from when [`tick`] was called, rather than
    /// from `start`.
    Delay,

    /// Skips missed ticks and tick on the next multiple of `period` from
    /// `start`.
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

/// An [`Interval`] allows you to wait on a sequence of instants with a certain duration between
/// each instant. 
/// 
/// # Type Parameter
/// 
/// - `D`: The underlying timer type that implements the [`Delay`] trait
pub struct Interval<D: Delay> {
    delay: Pin<Box<Sleep<D>>>,
    missed_tick_behavior: MissedTickBehavior,
    period: Duration,
}

impl<D> std::fmt::Debug for Interval<D>
where
    D: Delay + std::fmt::Debug,
    D::Instant: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interval")
            .field("delay", &self.delay)
            .field("missed_tick_behavior", &self.missed_tick_behavior)
            .field("period", &self.period)
            .finish()
    }
}

impl<D> Interval<D>
where
    D: Delay,
    D::Instant: Unpin,
{
    /// Creates new [`Interval`] that yields with interval of period. The first tick completes
    /// immediately. The default [`MissedTickBehavior`] is [`MissedTickBehavior::Burst`], but this
    /// can be configured by calling [`Interval::set_missed_tick_behavior`].
    /// 
    /// # Panics
    /// 
    /// This function will panic if `period` is zero.
    /// 
    /// # Examples
    /// 
    /// Creates a new [`Interval`] that yields every 100 milliseconds with smol's timer.
    /// 
    /// ```rust,no_run
    /// use std::time::Duration;
    /// use timer_kit::Interval;
    /// 
    /// let mut interval = Interval::<smol::Timer>::new(Duration::from_millis(100));
    /// 
    /// interval.tick().await;
    /// interval.tick().await;
    /// interval.tick().await;
    /// ```
    /// 
    /// Creates a new [`Interval`] that yields every 100 milliseconds with
    /// `fluvio_wasm_timer::Delay`.
    /// 
    /// ```rust,no_run
    /// use std::time::Duration;
    /// use timer_kit::Interval;
    /// 
    /// let mut interval = Interval::<fluvio_wasm_timer::Delay>::new(Duration::from_millis(100));
    /// 
    /// interval.tick().await;
    /// interval.tick().await;
    /// interval.tick().await;
    /// ```
    pub fn new(period: Duration) -> Self {
        assert!(period > Duration::new(0, 0), "period must be non-zero");
        Self {
            delay: Box::pin(Sleep::new(period)),
            missed_tick_behavior: MissedTickBehavior::default(),
            period,
        }
    }

    /// Creates new [`Interval`] that yields with interval of period with the first tick completing
    /// at start. The default [`MissedTickBehavior`] is [`MissedTickBehavior::Burst`], but this can
    /// be configured by calling [`Interval::set_missed_tick_behavior`].
    /// 
    /// # Panics
    /// 
    /// This function will panic if `period` is zero.
    /// 
    /// # Examples
    /// 
    /// Creates a new [`Interval`] that yields every 100 milliseconds with smol's timer.
    /// 
    /// ```rust,no_run
    /// use std::time::{Duration, Instant};
    /// use timer_kit::Interval;
    /// 
    /// let mut interval = Interval::<smol::Timer>::new_at(Instant::now(), Duration::from_millis(100));
    /// 
    /// interval.tick().await;
    /// interval.tick().await;
    /// interval.tick().await;
    /// ```
    /// 
    /// Creates a new [`Interval`] that yields every 100 milliseconds with
    /// `fluvio_wasm_timer::Delay`.
    /// 
    /// ```rust,no_run
    /// use std::time::Duration;
    /// use fluvio_wasm_timer::Instant;
    /// use timer_kit::Interval;
    /// 
    /// let mut interval = Interval::<fluvio_wasm_timer::Delay>::new_at(Instant::now(), Duration::from_millis(100));
    /// 
    /// interval.tick().await;
    /// interval.tick().await;
    /// interval.tick().await;
    /// ```
    pub fn new_at(start: D::Instant, period: Duration) -> Self {
        assert!(period > Duration::new(0, 0), "period must be non-zero");
        Self {
            delay: Box::pin(Sleep::new_until(start)),
            missed_tick_behavior: MissedTickBehavior::default(),
            period,
        }
    }

    /// Returns the [`MissedTickBehavior`] of the [`Interval`].
    pub fn missed_tick_behavior(&self) -> MissedTickBehavior {
        self.missed_tick_behavior
    }

    /// Sets the [`MissedTickBehavior`] of the [`Interval`].
    pub fn set_missed_tick_behavior(&mut self, behavior: MissedTickBehavior) {
        self.missed_tick_behavior = behavior;
    }

    /// Returns the period of the [`Interval`].
    pub fn period(&self) -> Duration {
        self.period
    }

    /// Polls the next tick of the [`Interval`].
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

    /// Completes the next tick of the [`Interval`].
    pub async fn tick(&mut self) -> D::Value {
        poll_fn(|cx| self.poll_tick(cx)).await
    }

    /// Resets the interval to complete one period after the current time.
    /// This method ignores [`MissedTickBehavior`] strategy.
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
