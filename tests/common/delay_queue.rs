//! `futures-timer` doesn't seems to work well with polling by the macro `assert_ready_some!`,
//! which is why some tests use `.await` instead.

#![allow(unused_imports)]

use std::time::Duration;

use futures::StreamExt;
use timer_kit::{Delay, Instant};

use super::*;

pub async fn single_immediate_delay<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    queue.insert_at("foo", D::Instant::now());

    timer_kit::sleep::<D>(Duration::from_millis(1)).await;

    assert_ready_some!(queue.next());
}

pub async fn multiple_immediate_delay<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    let start = D::Instant::now();
    queue.insert_at("1", start);
    queue.insert_at("2", start);
    queue.insert_at("3", start);

    let mut buffer = vec![];
    for _ in 0..3 {
        let val = queue.next().await.map(|expired| expired.into_inner());
        assert!(val.is_some());
        buffer.push(val.unwrap());
    }
    let end = D::Instant::now();
    let elapsed = end - start;
    assert!(elapsed < Duration::from_millis(50)); // TODO: this might be a large value on wasm

    let next = queue.next().await;
    assert!(next.is_none());

    buffer.sort();
    assert_eq!(buffer[0], "1");
    assert_eq!(buffer[1], "2");
    assert_eq!(buffer[2], "3");
}

pub async fn single_short_delay<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    let start = D::Instant::now();
    queue.insert_at("foo", start + Duration::from_millis(100));

    timer_kit::sleep::<D>(Duration::from_millis(50)).await;
    assert_pending!(queue.next());

    timer_kit::sleep::<D>(Duration::from_millis(50)).await;
    let entry = queue.next().await;
    let end = D::Instant::now();
    let elapsed = end - start;
    assert!(elapsed >= Duration::from_millis(100));
    assert!(entry.is_some());

    let next = queue.next().await;
    assert!(next.is_none());
}

pub async fn multi_delay_at_start<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let delays = vec![10, 20, 50, 100, 200, 500, 1000];
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    let start = D::Instant::now();
    for delay in &delays {
        queue.insert_at(*delay, start + Duration::from_millis(*delay));
    }

    timer_kit::sleep::<D>(Duration::from_millis(1)).await;
    assert_pending!(queue.next());

    let mut buffer = vec![];
    for _delay in &delays {
        let val = queue.next().await.map(|expired| expired.into_inner());
        #[cfg(not(target_arch = "wasm32"))] // TODO: disabled because time is not precise on wasm
        {
            let now = D::Instant::now();
            let elapsed = now - start;
            assert!(elapsed >= Duration::from_millis(*_delay)); 
        }
        assert!(val.is_some());
        buffer.push(val.unwrap());
    }

    assert_ready_none!(queue.next());
    assert_eq!(buffer, delays);
}

pub async fn insert_in_past_fires_immediately<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    queue.insert_at("foo", D::Instant::now());

    timer_kit::sleep::<D>(Duration::from_millis(100)).await;

    assert_ready_some!(queue.next());
    assert_ready_none!(queue.next());
}

pub async fn remove_entry<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    let key = queue.insert_at("foo", D::Instant::now() + Duration::from_millis(100));

    timer_kit::sleep::<D>(Duration::from_millis(50)).await;
    assert_pending!(queue.next());

    queue.remove(&key);
    timer_kit::sleep::<D>(Duration::from_millis(50)).await;
    assert_ready_none!(queue.next());
}

pub async fn reset_entry<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let key = queue.insert("foo", Duration::from_millis(100));

    timer_kit::sleep::<D>(Duration::from_millis(50)).await;
    assert_pending!(queue.next());

    let reset_start = D::Instant::now();
    queue.reset(&key, Duration::from_millis(100));
    timer_kit::sleep::<D>(Duration::from_millis(50)).await;
    assert_pending!(queue.next());

    let value = queue.next().await.unwrap().into_inner();
    let end = D::Instant::now();
    let elapsed = end - reset_start;
    assert!(elapsed >= Duration::from_millis(100));
    assert_eq!(value, "foo");
}

pub async fn reset_much_later<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let start = D::Instant::now();
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    let key = queue.insert_at("foo", start + Duration::from_millis(100));

    timer_kit::sleep::<D>(Duration::from_millis(5)).await;

    queue.reset_at(&key, start + Duration::from_millis(20));

    timer_kit::sleep::<D>(Duration::from_millis(30)).await;

    assert_ready_some!(queue.next());
}

pub async fn reset_twice<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let start = D::Instant::now();
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    let key = queue.insert_at("foo", start + Duration::from_millis(100));

    timer_kit::sleep::<D>(Duration::from_millis(5)).await;

    queue.reset_at(&key, start + Duration::from_millis(50));

    timer_kit::sleep::<D>(Duration::from_millis(20)).await;

    queue.reset_at(&key, start + Duration::from_millis(40));

    timer_kit::sleep::<D>(Duration::from_millis(20)).await;

    assert_ready_some!(queue.next());
}

pub async fn repeatedly_reset_entry_inserted_as_expired<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let start = D::Instant::now();
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    let key = queue.insert_at("foo", start);

    timer_kit::sleep::<D>(Duration::from_millis(5)).await;

    queue.reset_at(&key, start + Duration::from_millis(100));
    queue.reset_at(&key, start + Duration::from_millis(50));
    assert_pending!(queue.next());

    timer_kit::sleep::<D>(Duration::from_millis(50)).await;

    assert_ready_some!(queue.next());
    assert_ready_none!(queue.next());
}

pub async fn remove_expired_item<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    let key = queue.insert_at("foo", D::Instant::now());

    timer_kit::sleep::<D>(Duration::from_millis(10)).await;

    let entry = queue.remove(&key);
    assert_eq!(entry.into_inner(), "foo");

    assert_ready_none!(queue.next());
}

pub async fn remove_at_timer_wheel_threshold<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    let key1 = queue.insert_at("foo", D::Instant::now() + Duration::from_millis(64));
    let key2 = queue.insert_at("bar", D::Instant::now() + Duration::from_millis(64));

    timer_kit::sleep::<D>(Duration::from_millis(80)).await;

    let entry = assert_ready_some!(queue.next());

    match entry.into_inner() {
        "foo" => {
            let entry = queue.remove(&key2);
            assert_eq!(entry.into_inner(), "bar");
        }
        "bar" => {
            let entry = queue.remove(&key1);
            assert_eq!(entry.into_inner(), "foo");
        }
        _ => panic!("unexpected value"),
    }
}

pub async fn expires_before_last_insert<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let start = D::Instant::now();

    queue.insert_at("foo", start + Duration::from_millis(10_000));

    assert_pending!(queue.next());

    queue.insert_at("bar", start + Duration::from_millis(600));

    assert_pending!(queue.next());

    let entry = queue.next().await.unwrap().into_inner();
    let now = D::Instant::now();
    let elapsed = now - start;
    assert!(elapsed >= Duration::from_millis(600));
    assert_eq!(entry, "bar");
}

pub async fn multi_reset<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let start = D::Instant::now();
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let key1 = queue.insert_at("one", start + Duration::from_millis(100));
    let key2 = queue.insert_at("two", start + Duration::from_millis(200));

    assert_pending!(queue.next());

    queue.reset_at(&key1, start + Duration::from_millis(300));
    queue.reset_at(&key2, start + Duration::from_millis(400));
    queue.reset_at(&key1, start + Duration::from_millis(500));

    timer_kit::sleep::<D>(Duration::from_millis(110)).await;
    assert_pending!(queue.next());

    let entry = queue.next().await.unwrap();
    // TODO: disabled because time on wasm is not precise
    #[cfg(not(target_arch = "wasm32"))]
    {
        let now = D::Instant::now();
        let elapsed = now - start;
        assert!(elapsed >= Duration::from_millis(400) && elapsed < Duration::from_millis(500)); 
    }
    assert_eq!(entry.into_inner(), "two");

    let entry = queue.next().await.unwrap();
    // TODO: disabled because time on wasm is not precise
    #[cfg(not(target_arch = "wasm32"))]
    {
        let now = D::Instant::now();
        let elapsed = now - start;
        assert!(elapsed >= Duration::from_millis(500)); 
    }
    assert_eq!(entry.into_inner(), "one");
}

pub async fn expire_first_key_when_reset_to_expire_earlier<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let start = D::Instant::now();

    let key1 = queue.insert_at("one", start + Duration::from_millis(100));
    queue.insert_at("two", start + Duration::from_millis(200));

    assert_pending!(queue.next());

    queue.reset_at(&key1, start + Duration::from_millis(50));

    timer_kit::sleep::<D>(Duration::from_millis(60)).await;

    let entry = assert_ready_some!(queue.next());

    assert_eq!(entry.into_inner(), "one");
}

pub async fn expire_second_key_when_reset_to_expire_earlier<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let start = D::Instant::now();

    queue.insert_at("one", start + Duration::from_millis(100));
    let key2 = queue.insert_at("two", start + Duration::from_millis(200));

    assert_pending!(queue.next());

    queue.reset_at(&key2, start + Duration::from_millis(50));

    timer_kit::sleep::<D>(Duration::from_millis(60)).await;

    let entry = assert_ready_some!(queue.next());

    assert_eq!(entry.into_inner(), "two");
}

pub async fn reset_first_expiring_item_to_expire_later<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let start = D::Instant::now();

    let key1 = queue.insert_at("one", start + Duration::from_millis(100));
    queue.insert_at("two", start + Duration::from_millis(200));

    assert_pending!(queue.next());

    queue.reset_at(&key1, start + Duration::from_millis(300));

    let entry = queue.next().await.unwrap();
    let now = D::Instant::now();
    let elapsed = now - start;
    assert!(elapsed >= Duration::from_millis(200) && elapsed < Duration::from_millis(300));

    assert_eq!(entry.into_inner(), "two");
}

pub async fn insert_before_first_after_poll<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let start = D::Instant::now();

    queue.insert_at("one", start + Duration::from_millis(200));

    assert_pending!(queue.next());

    queue.insert_at("two", start + Duration::from_millis(100));

    timer_kit::sleep_until::<D>(start + Duration::from_millis(90)).await;

    assert_pending!(queue.next());

    timer_kit::sleep_until::<D>(start + Duration::from_millis(101)).await;

    let entry = assert_ready_some!(queue.next());
    assert_eq!(entry.into_inner(), "two");
}

pub async fn insert_after_ready_poll<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    queue.insert("one", Duration::from_millis(100));

    timer_kit::sleep::<D>(Duration::from_millis(101)).await;

    let entry = assert_ready_some!(queue.next());
    assert_eq!(entry.into_inner(), "one");

    queue.insert("two", Duration::from_millis(100));
}

pub async fn reset_later_after_slot_starts<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let start = D::Instant::now();

    let key1 = queue.insert_at("one", start + Duration::from_millis(100));

    assert_pending!(queue.next());

    timer_kit::sleep::<D>(Duration::from_millis(80)).await;

    queue.reset_at(&key1, start + Duration::from_millis(120));

    assert_pending!(queue.next());

    timer_kit::sleep_until::<D>(start + Duration::from_millis(110)).await;

    assert_pending!(queue.next());

    timer_kit::sleep::<D>(Duration::from_millis(11)).await;

    let entry = assert_ready_some!(queue.next());
    assert_eq!(entry.into_inner(), "one");
}

pub async fn reset_insert_expired<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let start = D::Instant::now();

    let key = queue.insert_at("one", start - Duration::from_millis(100));

    queue.reset_at(&key, start + Duration::from_millis(100));

    assert_eq!(queue.len(), 1);

    let entry = queue.next().await.unwrap();
    let now = D::Instant::now();
    let elapsed = now - start;
    assert!(elapsed >= Duration::from_millis(100));
    assert_eq!(entry.into_inner(), "one");

    assert_eq!(queue.len(), 0);
}

pub async fn reset_earlier_after_slot_starts<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let start = D::Instant::now();

    let key1 = queue.insert_at("one", start + Duration::from_millis(200));

    assert_pending!(queue.next());

    timer_kit::sleep::<D>(Duration::from_millis(80)).await;

    queue.reset_at(&key1, start + Duration::from_millis(120));

    assert_pending!(queue.next());

    timer_kit::sleep_until::<D>(start + Duration::from_millis(110)).await;

    assert_pending!(queue.next());

    timer_kit::sleep::<D>(Duration::from_millis(11)).await;

    let entry = assert_ready_some!(queue.next());
    assert_eq!(entry.into_inner(), "one");
}

pub async fn insert_in_past_after_poll_fires_immediately<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let start = D::Instant::now();

    queue.insert_at("foo", start + Duration::from_millis(200));

    assert_pending!(queue.next());

    timer_kit::sleep::<D>(Duration::from_millis(80)).await;

    assert_pending!(queue.next());
    queue.insert_at("bar", start + Duration::from_millis(40));

    let entry = queue.next().await.unwrap().into_inner();
    assert_eq!(entry, "bar");
}

pub async fn delay_queue_poll_expired_when_empty<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    let key = queue.insert("foo", Duration::from_millis(100));
    assert_pending!(queue.next());

    queue.remove(&key);
    assert_ready_none!(queue.next());
}

pub async fn compact_expire_empty<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let start = D::Instant::now();

    queue.insert_at("one", start + Duration::from_millis(100));
    queue.insert_at("two", start + Duration::from_millis(100));

    for _ in 0..2 {
        let entry = queue.next().await.unwrap().into_inner();
        assert!(entry == "one" || entry == "two");
    }
    let now = D::Instant::now();
    let elapsed = now - start;
    assert!(elapsed >= Duration::from_millis(100));

    queue.compact();

    assert_eq!(queue.len(), 0);
    assert_eq!(queue.capacity(), 0);
}

pub async fn compact_remove_empty<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let start = D::Instant::now();

    let key1 = queue.insert_at("one", start + Duration::from_millis(100));
    let key2 = queue.insert_at("two", start + Duration::from_millis(100));

    queue.remove(&key1);
    queue.remove(&key2);

    queue.compact();

    assert_eq!(queue.len(), 0);
    assert_eq!(queue.capacity(), 0);
}

pub async fn compact_remove_remapped_keys<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let start = D::Instant::now();

    queue.insert_at("one", start + Duration::from_millis(100));
    queue.insert_at("two", start + Duration::from_millis(100));

    let key3 = queue.insert_at("three", start + Duration::from_millis(200));
    let key4 = queue.insert_at("four", start + Duration::from_millis(200));

    for _ in 0..2 {
        let entry = queue.next().await.unwrap().into_inner();
        assert!(entry == "one" || entry == "two");
    }
    let now = D::Instant::now();
    let elapsed = now - start;
    assert!(elapsed >= Duration::from_millis(100) && elapsed < Duration::from_millis(200));

    queue.compact();

    queue.insert_at("five", start + Duration::from_millis(100));

    let removed3 = queue.remove(&key3);
    let removed4 = queue.remove(&key4);

    assert_eq!(removed3.into_inner(), "three");
    assert_eq!(removed4.into_inner(), "four");

    queue.compact();

    assert_eq!(queue.len(), 1);
    assert_eq!(queue.capacity(), 1);
}

pub async fn compact_change_deadline<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let mut queue = timer_kit::DelayQueue::<D, _>::new();

    let initial_start = D::Instant::now();

    queue.insert_at("one", initial_start + Duration::from_millis(100));
    queue.insert_at("two", initial_start + Duration::from_millis(100));

    queue.insert_at("three", initial_start + Duration::from_millis(200));
    let key4 = queue.insert_at("four", initial_start + Duration::from_millis(200));

    for _ in 0..2 {
        let expired = queue.next().await;
        assert!(expired.is_some());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let now = D::Instant::now();
        let elapsed = now - initial_start;
        assert!(elapsed >= Duration::from_millis(100) && elapsed < Duration::from_millis(200));
    }

    queue.compact();

    let compact_start = D::Instant::now();

    queue.insert_at("five", compact_start + Duration::from_millis(100));
    let key6 = queue.insert_at("six", compact_start + Duration::from_millis(100));

    queue.reset_at(&key4, compact_start + Duration::from_millis(200));
    queue.reset_at(&key6, compact_start + Duration::from_millis(200));

    for _ in 0..2 {
        let expired = queue.next().await;
        assert!(expired.is_some());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let now = D::Instant::now();
        let elapsed = now - compact_start;
        assert!(elapsed >= Duration::from_millis(100) && elapsed < Duration::from_millis(200));
    }

    timer_kit::sleep::<D>(Duration::from_millis(101)).await;

    for _ in 0..2 {
        let expired = queue.next().await;
        assert!(expired.is_some());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let now = D::Instant::now();
        let elapsed = now - compact_start;
        assert!(elapsed >= Duration::from_millis(200));
    }

    assert_ready_none!(queue.next());
}

pub async fn remove_after_compact<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let start = D::Instant::now();
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    let key = queue.insert_at("one", start + Duration::from_millis(100));
    queue.insert_at("two", start + Duration::from_millis(100));
    queue.remove(&key);
}

pub async fn panic_on_remove_of_nonexistent_key<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let start = D::Instant::now();
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    let key = queue.insert_at("one", start + Duration::from_millis(100));
    queue.insert_at("two", start + Duration::from_millis(100));
    queue.remove(&key);

    // This should panic
    queue.remove(&key);
}

pub async fn panic_on_remove_after_compact_poll<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let start = D::Instant::now();
    let mut queue = timer_kit::DelayQueue::<D, _>::new();
    let key1 = queue.insert_at("one", start + Duration::from_millis(100));
    queue.insert_at("two", start + Duration::from_millis(200));

    timer_kit::sleep::<D>(Duration::from_millis(101)).await;

    let entry = assert_ready_some!(queue.next());
    assert_eq!(entry.into_inner(), "one");

    queue.compact();

    // This should panic
    queue.remove(&key1);
}
