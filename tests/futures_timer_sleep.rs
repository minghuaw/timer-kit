#[macro_use]
mod macros;

mod common;

cfg_not_wasm32! {
    cfg_futures_timer! {
        #[futures_test::test]
        async fn immediate_sleep() {
            common::sleep::immediate_sleep::<futures_timer::Delay>().await;
        }

        #[futures_test::test]
        async fn short_sleep() {
            common::sleep::short_sleep::<futures_timer::Delay>().await;
        }

        #[futures_test::test]
        async fn reset() {
            common::sleep::reset::<futures_timer::Delay>().await;
        }
    }
}