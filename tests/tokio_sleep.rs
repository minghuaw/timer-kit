#[macro_use]
mod macros;

mod util;

cfg_tokio! {
    #[tokio::test]
    async fn immediate_sleep() {
        util::sleep::immediate_sleep::<tokio::time::Sleep>().await;
    }

    #[tokio::test]
    async fn short_sleep() {
        util::sleep::short_sleep::<tokio::time::Sleep>().await;
    }

    #[tokio::test]
    async fn reset() {
        util::sleep::reset::<tokio::time::Sleep>().await;
    }
}