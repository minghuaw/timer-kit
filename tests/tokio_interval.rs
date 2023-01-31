#[macro_use]
mod macros;

mod util;

cfg_tokio! {

    #[tokio::test]
    #[should_panic]
    async fn interval_zero_duration() {
        util::interval::interval_zero_duration::<tokio::time::Sleep>().await;
    }

    #[tokio::test]
    async fn burst() {
        util::interval::burst::<tokio::time::Sleep>().await;
    }

    #[tokio::test]
    async fn delay() {
        util::interval::delay::<tokio::time::Sleep>().await;
    }

    #[tokio::test]
    async fn skip() {
        util::interval::skip::<tokio::time::Sleep>().await;
    }

    #[tokio::test]
    async fn reset() {
        util::interval::reset::<tokio::time::Sleep>().await;
    }
}