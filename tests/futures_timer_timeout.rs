#[macro_use]
mod macros;

mod common;

cfg_not_wasm32! {
    cfg_futures_timer! {
        #[futures_test::test]
        async fn simultaneous_deadline_future_completion() {
            common::timeout::simultaneous_deadline_future_completion::<futures_timer::Delay>().await;
        }

        #[futures_test::test]
        async fn completed_future_past_deadline() {
            common::timeout::completed_future_past_deadline::<futures_timer::Delay>().await;
        }

        #[futures_test::test]
        async fn future_and_deadline_in_future() {
            common::timeout::future_and_deadline_in_future::<futures_timer::Delay>().await;
        }

        #[futures_test::test]
        async fn deadline_future_elapses() {
            common::timeout::deadline_future_elapses::<futures_timer::Delay>().await;
        }
    }
}