#![allow(dead_code, unused_imports)]

use std::time::Duration;

use timer_kit::{Delay, Instant};

use super::*;

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
    timer_kit::sleep::<D>(Duration::from_millis(10)).await;
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