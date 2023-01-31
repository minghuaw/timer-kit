#![allow(dead_code, unused_imports)]

use std::time::Duration;

use timer_kit::{Delay, Instant};

use super::*;

pub async fn simultaneous_deadline_future_completion<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let timeout = timer_kit::timeout_at::<D, _>(D::Instant::now(), async {});
    assert_ready_ok!(timeout);
}

pub async fn completed_future_past_deadline<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let timeout =
        timer_kit::timeout_at::<D, _>(D::Instant::now() - Duration::from_millis(100), async {});
    assert_ready_ok!(timeout);
}

pub async fn future_and_deadline_in_future<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let (tx, rx) = futures::channel::oneshot::channel::<()>();
    let timeout = timer_kit::timeout_at::<D, _>(D::Instant::now() + Duration::from_millis(100), rx);
    let mut timeout = Box::pin(timeout); // This is only needed for the assert_pending! macro

    assert_pending!(&mut timeout);
    timer_kit::sleep::<D>(Duration::from_millis(50)).await;
    assert_pending!(&mut timeout);

    tx.send(()).unwrap();
    assert_ready_ok!(timeout);
}

pub async fn deadline_future_elapses<D>()
where
    D: Delay,
    D::Instant: Unpin,
{
    let deadline = D::Instant::now() + Duration::from_millis(100);
    let timeout = timer_kit::timeout_at::<D, _>(deadline, never());

    timer_kit::sleep::<D>(Duration::from_millis(101)).await;

    assert_ready_err!(timeout);
}

// TODO: this is impossible
// pub async fn timeout_is_not_exhausted_by_future<D>()
// where
//     D: Delay,
//     D::Instant: Unpin,
// {
//     struct PendingEveryTenPolls {
//         count: usize,
//     }

//     impl std::future::Future for &mut PendingEveryTenPolls {
//         type Output = ();

//         fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
//             let this = self.get_mut();
//             this.count += 1;
//             if this.count % 10 == 0 {
//                 cx.waker().wake_by_ref();
//                 std::task::Poll::Pending
//             } else {
//                 std::task::Poll::Ready(())
//             }

//             // std::task::Poll::Ready(())
//         }
//     }

//     let one_milli = Duration::from_millis(1);
//     let fut = timer_kit::timeout::<D, _>(one_milli, async {
//         let mut buffer = [0u8; 1];
//         let mut fut = PendingEveryTenPolls { count: 0 };
//         loop {
//             // use futures::io::AsyncReadExt;
//             // let _ = futures::io::empty().read(&mut buffer).await;

//             // use tokio::io::AsyncReadExt;
//             // let _ = tokio::io::empty().read(&mut buffer).await;

//             // (&mut fut).await;
//         }
//     });

//     assert!(fut.await.is_err());
// }
