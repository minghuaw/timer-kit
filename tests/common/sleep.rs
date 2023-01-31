#![allow(dead_code, unused_imports)]

use std::time::Duration;

use timer_kit::{Delay, Instant};

use super::*;

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

// TODO: how to test this without actually sleeping for days/weeks/months/years?
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

pub async fn reset<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let deadline = D::Instant::now() + Duration::from_millis(100);
    let mut sleep = timer_kit::sleep_until::<D>(deadline);
    timer_kit::sleep::<D>(Duration::from_millis(50)).await;
    assert_pending!(&mut sleep);

    // Reset by 100ms
    let new_deadline = D::Instant::now() + Duration::from_millis(100);
    sleep.reset(new_deadline);

    // Should return pending at the original deadline
    timer_kit::sleep::<D>(Duration::from_millis(50)).await;
    assert_pending!(&mut sleep);

    // Should return ready at the new deadline
    timer_kit::sleep::<D>(Duration::from_millis(50)).await;
    assert_ready!(sleep);
}
