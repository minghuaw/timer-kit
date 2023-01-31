#[macro_use]
mod macros;

cfg_tokio! {
    mod util;

    #[tokio::test]
    #[should_panic]
    async fn interval_zero_duration() {
        util::interval_zero_duration::<tokio::time::Sleep>().await;
    }

    #[tokio::test]
    async fn burst() {
        util::burst::<tokio::time::Sleep>().await;
    }

    #[tokio::test]
    async fn delay() {
        util::delay::<tokio::time::Sleep>().await;
    }

    #[tokio::test]
    async fn skip() {
        util::skip::<tokio::time::Sleep>().await;
    }

    #[tokio::test]
    async fn reset() {
        util::reset::<tokio::time::Sleep>().await;
    }
}