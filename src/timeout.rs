use std::{future::Future, pin::Pin, task::Poll, time::Duration};

use pin_project_lite::pin_project;

use crate::{error::Elapsed, Delay};

/// Creates a new `Timeout` with a specified duration.
/// 
/// # Example
/// 
/// Creates a timeout with smol's timer
/// 
/// ```rust,no_run
/// use std::time::Duration;
/// use timer_kit::timeout;
/// 
/// let result = timeout::<smol::Timer, _>(Duration::from_millis(100), async { }).await;
/// ```
/// 
/// Creates a timeout with `fluvio_wasm_timer::Delay`
/// 
/// ```rust,no_run
/// use std::time::Duration;
/// use timer_kit::timeout;
/// 
/// let result = timeout::<fluvio_wasm_timer::Delay, _>(Duration::from_millis(100), async { }).await;
/// ```
pub fn timeout<D, Fut>(duration: Duration, fut: Fut) -> Timeout<D, Fut>
where
    D: Delay,
    Fut: Future,
{
    Timeout::new(duration, fut)
}

/// Creates a new `Timeout` with a specified deadline.
/// 
/// # Example
/// 
/// Creates a timeout with smol's timer
/// 
/// ```rust,no_run
/// use std::time::{Duration, Instant};
/// use timer_kit::timeout_at;
/// 
/// let result = timeout_at::<smol::Timer, _>(Instant::now() + Duration::from_millis(100), async { }).await;
/// ```
/// 
/// Creates a timeout with `fluvio_wasm_timer::Delay`
/// 
/// ```rust,no_run
/// use std::time::{Duration};
/// use fluent_wasm_timer::Instant;
/// use timer_kit::timeout_at;
/// 
/// let result = timeout_at::<fluvio_wasm_timer::Delay, _>(Instant::now() + Duration::from_millis(100), async { }).await;
/// ```
pub fn timeout_at<D, Fut>(deadline: D::Instant, fut: Fut) -> Timeout<D, Fut>
where
    D: Delay,
    Fut: Future,
{
    Timeout::new_at(deadline, fut)
}

pin_project! {
    /// A timeout future.
    ///
    /// It returns `<Fut as Future>::Output` if the future completes before the timeout or
    /// [`Elapsed`] if the timeout elapses before the future completes.
    ///
    /// # Warning - Exhaustion
    ///
    /// This future is not able to avoid exhaustion if the future never completes and never returns
    /// `Pending`. The user should ensure that the `Fut` future is able to return `Pending` at some
    /// point to avoid exhaustion.
    pub struct Timeout<D, Fut> {
        #[pin]
        delay: D,

        #[pin]
        future: Fut,
    }
}

impl<D, Fut> Timeout<D, Fut>
where
    D: Delay,
    Fut: Future,
{
    /// Creates a new `Timeout` with a specified duration.
    ///
    /// # Example
    ///
    /// Creates a timeout with smol's timer
    /// 
    /// ```rust,no_run
    /// use std::time::Duration;
    /// use timer_kit::Timeout;
    /// 
    /// let result = Timeout::<smol::Timer, _>::new(Duration::from_millis(100), async {}).await;
    /// ```
    /// 
    /// Creates a timeout with `fluvio_wasm_timer::Delay`
    /// 
    /// ```rust,no_run
    /// use std::time::Duration;
    /// use timer_kit::Timeout;
    /// 
    /// let result = Timeout::<fluvio_wasm_timer::Delay, _>::new(Duration::from_millis(100), async {}).await;
    /// ```
    pub fn new(duration: Duration, future: Fut) -> Self {
        Self {
            delay: D::delay(duration),
            future,
        }
    }

    /// Creates a new `Timeout` with a specified deadline.
    ///
    /// # Example
    ///
    /// Creates a timeout with smol's timer
    ///
    /// ```rust,no_run
    /// use std::time::{Duration, Instant};
    /// use timer_kit::Timeout;
    ///
    /// let result = Timeout::<smol::Timer, _>::new_at(Instant::now() + Duration::from_millis(100), async {}).await;
    /// ```
    ///
    /// Creates a timeout with `fluvio_wasm_timer::Delay`
    /// 
    /// ```rust,no_run
    /// use std::time::Duration;
    /// use fluent_wasm_timer::Instant;
    /// use timer_kit::Timeout;
    /// 
    /// let result = Timeout::<fluvio_wasm_timer::Delay, _>::new_at(Instant::now() + Duration::from_millis(100), async {}).await;
    /// ```
    pub fn new_at(deadline: D::Instant, future: Fut) -> Self {
        Self {
            delay: D::delay_until(deadline),
            future,
        }
    }
}

impl<D, Fut> Future for Timeout<D, Fut>
where
    D: Delay,
    Fut: Future,
{
    type Output = Result<Fut::Output, Elapsed>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
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
