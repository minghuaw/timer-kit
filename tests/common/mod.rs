#![allow(dead_code, unused_macros)]

macro_rules! assert_ready {
    ($fut:ident) => {
        let pinned = std::pin::Pin::new(&mut $fut);
        assert!(futures_util::poll!(pinned).is_ready());
    };
}

macro_rules! assert_pending {
    ($fut:ident) => {
        let pinned = std::pin::Pin::new(&mut $fut);
        assert!(futures_util::poll!(pinned).is_pending());
    };
}

macro_rules! assert_interval_poll_ready {
    ($interval:ident) => {
        let mut fut = futures_util::future::poll_fn(|cx| $interval.poll_tick(cx));
        assert_ready!(fut);
    };
}

macro_rules! assert_interval_poll_pending {
    ($interval:ident) => {
        let mut fut = futures_util::future::poll_fn(|cx| $interval.poll_tick(cx));
        assert_pending!(fut);
    };
}

macro_rules! assert_ready_ok {
    ($fut:ident) => {
        futures_util::pin_mut!($fut);
        match futures_util::poll!($fut) {
            std::task::Poll::Ready(Ok(_)) => {}
            std::task::Poll::Ready(Err(_)) => panic!("expected Ok, got Err"),
            std::task::Poll::Pending => panic!("expected Ok, got Pending"),
        }
    };
}

macro_rules! assert_ready_err {
    ($fut:ident) => {
        futures_util::pin_mut!($fut);
        match futures_util::poll!($fut) {
            std::task::Poll::Ready(Ok(_)) => panic!("expected Err, got Ok"),
            std::task::Poll::Ready(Err(_)) => {}
            std::task::Poll::Pending => panic!("expected Err, got Pending"),
        }
    };
}

pub fn never() -> Never {
    Never { _sealed: () }
}

pub struct Never {
    _sealed: (),
}

impl std::future::Future for Never {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::task::Poll::Pending
    }
}

pub mod interval;
pub mod sleep;
pub mod timeout;