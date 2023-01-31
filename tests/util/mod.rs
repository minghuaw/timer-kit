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

pub mod interval;
pub mod sleep;