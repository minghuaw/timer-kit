use std::{pin::Pin, time::{Duration}, future::Future};

use crate::{Delay, Instant};

/// Creates a new `Sleep` that completes after the specified duration.
/// 
/// # Example
/// 
/// Creates a sleep with smol's timer
/// 
/// ```rust,no_run
/// use std::time::Duration;
/// use timer_kit::sleep;
/// 
/// sleep::<smol::Timer>(Duration::from_millis(100)).await;
/// ```
/// 
/// Creates a sleep with `fluvio_wasm_timer::Delay`
/// 
/// ```rust,no_run
/// use std::time::Duration;
/// use timer_kit::sleep;
/// 
/// sleep::<fluvio_wasm_timer::Delay>(Duration::from_millis(100)).await;
/// ```
pub fn sleep<D>(duration: Duration) -> Sleep<D> 
where
    D: Delay,
{
    Sleep::new(duration)
}

/// Creates a new `Sleep` that completes at the specified deadline
/// 
/// # Example
/// 
/// Creates a sleep with smol's timer
/// 
/// ```rust,no_run
/// use std::time::{Duration, Instant};
/// use timer_kit::sleep_until;
/// 
/// sleep_until::<smol::Timer>(Instant::now() + Duration::from_millis(100)).await;
/// ```
/// 
/// Creates a sleep with `fluvio_wasm_timer::Delay`
/// 
/// ```rust,no_run
/// use std::time::{Duration};
/// use fluent_wasm_timer::Instant;
/// use timer_kit::sleep_until;
/// 
/// sleep_until::<fluvio_wasm_timer::Delay>(Instant::now() + Duration::from_millis(100)).await;
/// ```
pub fn sleep_until<D>(deadline: D::Instant) -> Sleep<D> 
where
    D: Delay,
{
    Sleep::new_until(deadline)
}

/// A future that completes after a specified duration.
/// 
/// This future calls `Delay::poll_elapsed` internally.
/// 
/// # Type Parameter
/// 
/// - `D`: The underlying timer type that implements the [`Delay`] trait
#[derive(Debug)]
pub struct Sleep<D: Delay> {
    delay: Pin<Box<D>>,
    deadline: D::Instant,
}

impl<D> Sleep<D>
where
    D: Delay,
{
    /// Creates a new `Sleep` that completes after the specified duration.
    /// 
    /// # Example
    /// 
    /// Creates a sleep with smol's timer
    /// 
    /// ```rust,no_run
    /// use std::time::Duration;
    /// use timer_kit::Sleep;
    /// 
    /// let sleep = Sleep::<smol::Timer>::new(Duration::from_millis(100));
    /// sleep.await;
    /// ```
    /// 
    /// Creates a sleep with `fluvio_wasm_timer::Delay`
    /// 
    /// ```rust,no_run
    /// use std::time::Duration;
    /// use timer_kit::Sleep;
    /// 
    /// let sleep = Sleep::<fluvio_wasm_timer::Delay>::new(Duration::from_millis(100));
    /// sleep.await;
    /// ```
    pub fn new(duration: Duration) -> Self 
    {
        let delay = Box::pin(D::delay(duration));
        let deadline = delay.deadline().unwrap_or(D::Instant::now() + duration);
        Self {
            delay,
            deadline,
        }
    }

    /// Creates a new `Sleep` that completes at the specified deadline
    /// 
    /// # Example
    /// 
    /// Creates a sleep with smol's timer
    /// 
    /// ```rust,no_run
    /// use std::time::{Duration, Instant};
    /// use timer_kit::Sleep;
    /// 
    /// let sleep = Sleep::<smol::Timer>::new_until(Instant::now() + Duration::from_millis(100));
    /// sleep.await;
    /// ```
    /// 
    /// Creates a sleep with `fluvio_wasm_timer::Delay`
    /// 
    /// ```rust,no_run
    /// use std::time::{Duration};
    /// use fluent_wasm_timer::Instant;
    /// use timer_kit::Sleep;
    /// 
    /// let sleep = Sleep::<fluvio_wasm_timer::Delay>::new_until(Instant::now() + Duration::from_millis(100));
    /// sleep.await;
    /// ```
    pub fn new_until(deadline: D::Instant) -> Self {
        Self {
            delay: Box::pin(D::delay_until(deadline)),
            deadline,
        }
    }

    /// Reset the `Sleep` to a new deadline
    pub fn reset(&mut self, deadline: D::Instant) {
        self.deadline = deadline;
        self.delay.as_mut().reset(deadline);
    }

    /// Gets the deadline
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
