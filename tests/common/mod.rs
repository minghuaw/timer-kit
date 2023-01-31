#![allow(dead_code, unused_macros)]

macro_rules! assert_ready {
    ($fut:expr) => {
        {
            let mut fut = $fut;
            let pinned = std::pin::Pin::new(&mut fut);
            match futures_util::poll!(pinned) {
                std::task::Poll::Ready(val) => val,
                std::task::Poll::Pending => panic!("expected Ready, got Pending"),
            }
        }
    };
}

macro_rules! assert_pending {
    ($fut:expr) => {
        {
            let mut fut = $fut;
            let pinned = std::pin::Pin::new(&mut fut);
            match futures_util::poll!(pinned) {
                std::task::Poll::Ready(_) => panic!("expected Pending, got Ready"),
                std::task::Poll::Pending => {}
            }
        }
    };
}

macro_rules! assert_interval_poll_ready {
    ($interval:ident) => {
        let fut = futures_util::future::poll_fn(|cx| $interval.poll_tick(cx));
        assert_ready!(fut);
    };
}

macro_rules! assert_interval_poll_pending {
    ($interval:ident) => {
        let fut = futures_util::future::poll_fn(|cx| $interval.poll_tick(cx));
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

macro_rules! assert_ready_some {
    ($fut:expr) => {
        {
            let fut = $fut;
            futures_util::pin_mut!(fut);
            match futures_util::poll!(fut) {
                std::task::Poll::Ready(Some(val)) => val,
                std::task::Poll::Ready(None) => panic!("expected Some, got None"),
                std::task::Poll::Pending => panic!("expected Some, got Pending"),
            }
        }
    };
}

macro_rules! assert_ready_none {
    ($fut:expr) => {
        {
            let fut = $fut;
            futures_util::pin_mut!(fut);
            match futures_util::poll!(fut) {
                std::task::Poll::Ready(Some(_)) => panic!("expected None, got Some"),
                std::task::Poll::Ready(None) => {}
                std::task::Poll::Pending => panic!("expected None, got Pending"),
            }
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
pub mod delay_queue;