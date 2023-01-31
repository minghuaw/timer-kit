#![allow(dead_code)]

use std::time::Duration;
use futures_util::{poll, pin_mut, future::poll_fn};

use timer_kit::{Delay, Instant};

// =============================================================================
// Interval tests
// =============================================================================

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

pub async fn interval_zero_duration<D>() 
where
    D: Delay,
    D::Instant: Unpin,
{
    let _ = timer_kit::interval::<D>(Duration::from_millis(0));
}

pub async fn burst<D>() 
where
    D: Delay,
    D::Instant: Unpin,
{
    let start = D::Instant::now() + Duration::from_millis(100);
    let mut interval = timer_kit::interval_at::<D>(start, Duration::from_millis(300));
    interval.set_missed_tick_behavior(timer_kit::MissedTickBehavior::Burst);

    // Give a 1ms buffer
    timer_kit::sleep::<D>(Duration::from_millis(1)).await;
    assert_interval_poll_pending!(interval);

    // Interval starts at 100ms
    timer_kit::sleep::<D>(Duration::from_millis(100)).await;
    assert_interval_poll_ready!(interval);

    // Miss two tick
    timer_kit::sleep::<D>(Duration::from_millis(700)).await;
    assert_interval_poll_ready!(interval);
    assert_interval_poll_ready!(interval);
    assert_interval_poll_pending!(interval);

    timer_kit::sleep::<D>(Duration::from_millis(200)).await;
    assert_interval_poll_ready!(interval);
    assert_interval_poll_pending!(interval);
}

pub async fn delay<D>() 
where
    D: Delay,
    D::Instant: Unpin,
{
    let start = D::Instant::now() + Duration::from_millis(100);
    let mut interval = timer_kit::interval_at::<D>(start, Duration::from_millis(300));
    interval.set_missed_tick_behavior(timer_kit::MissedTickBehavior::Delay);

    // Give a 1ms buffer
    timer_kit::sleep::<D>(Duration::from_millis(1)).await;
    assert_interval_poll_pending!(interval);

    // Interval starts at 100ms
    timer_kit::sleep::<D>(Duration::from_millis(100)).await;
    assert_interval_poll_ready!(interval);

    // Miss two tick
    timer_kit::sleep::<D>(Duration::from_millis(700)).await;
    assert_interval_poll_ready!(interval);
    assert_interval_poll_pending!(interval);

    // Next tick is delayed until 1100ms
    timer_kit::sleep::<D>(Duration::from_millis(200)).await;
    assert_interval_poll_pending!(interval);
    timer_kit::sleep::<D>(Duration::from_millis(100)).await;
    assert_interval_poll_ready!(interval);

    // Next tick is delayed until 1400ms
    timer_kit::sleep::<D>(Duration::from_millis(100)).await;
    assert_interval_poll_pending!(interval);
    timer_kit::sleep::<D>(Duration::from_millis(200)).await;
    assert_interval_poll_ready!(interval);
}

pub async fn skip<D>() 
where
    D: Delay,
    D::Instant: Unpin,
{
    let start = D::Instant::now() + Duration::from_millis(100);
    let mut interval = timer_kit::interval_at::<D>(start, Duration::from_millis(300));
    interval.set_missed_tick_behavior(timer_kit::MissedTickBehavior::Skip);

    // Give a 1ms buffer
    timer_kit::sleep::<D>(Duration::from_millis(1)).await;
    assert_interval_poll_pending!(interval);

    // Interval starts at 100ms
    timer_kit::sleep::<D>(Duration::from_millis(100)).await;
    assert_interval_poll_ready!(interval);

    // Miss two tick
    timer_kit::sleep::<D>(Duration::from_millis(700)).await;
    assert_interval_poll_ready!(interval);
    assert_interval_poll_pending!(interval);
    
    timer_kit::sleep::<D>(Duration::from_millis(200)).await;
    assert_interval_poll_ready!(interval);
    assert_interval_poll_pending!(interval);

    timer_kit::sleep::<D>(Duration::from_millis(300)).await;
    assert_interval_poll_ready!(interval);
}

pub async fn reset<D>() 
where
    D: Delay,
    D::Instant: Unpin,
{
    let start = D::Instant::now() + Duration::from_millis(100);
    let mut interval = timer_kit::interval_at::<D>(start, Duration::from_millis(300));

    // Give a 1ms buffer
    timer_kit::sleep::<D>(Duration::from_millis(1)).await;
    assert_interval_poll_pending!(interval);

    // Interval starts at 100ms
    timer_kit::sleep::<D>(Duration::from_millis(100)).await;
    assert_interval_poll_ready!(interval);

    timer_kit::sleep::<D>(Duration::from_millis(300)).await;
    assert_interval_poll_ready!(interval);

    timer_kit::sleep::<D>(Duration::from_millis(50)).await;
    interval.reset();

    timer_kit::sleep::<D>(Duration::from_millis(250)).await;
    assert_interval_poll_pending!(interval);

    timer_kit::sleep::<D>(Duration::from_millis(50)).await;
    assert_interval_poll_ready!(interval);

    timer_kit::sleep::<D>(Duration::from_millis(300)).await;
    assert_interval_poll_ready!(interval);
}

// =============================================================================
// Sleep tests
// =============================================================================

pub async fn immediate_sleep<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let deadline = D::Instant::now();
    timer_kit::sleep_until::<D>(deadline).await;
    let now = D::Instant::now();
    assert!(now - deadline < Duration::from_millis(10));
}

pub async fn short_sleep<D>() 
where
    D: Delay,
    D::Instant: Unpin,
{
    let deadline = D::Instant::now() + Duration::from_millis(100);
    timer_kit::sleep_until::<D>(deadline).await;
    let now = D::Instant::now();
    assert!(now - deadline < Duration::from_millis(10));
}

// pub async fn long_sleep<D>() 
// where
//     D: Delay,
//     D::Instant: Unpin,
// {
//     let deadline = D::Instant::now() + Duration::from_millis(10_000);
//     timer_kit::sleep_until::<D>(deadline).await;
//     let now = D::Instant::now();
//     assert!(now - deadline < Duration::from_millis(10));
// }

// =============================================================================
// Timeout tests
// =============================================================================

// =============================================================================
// DelayQueue tests
// =============================================================================