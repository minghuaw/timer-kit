#[macro_use]
mod macros;

mod common;

cfg_not_wasm32! {
    cfg_futures_timer! {
        #[futures_test::test]
        #[should_panic]
        async fn interval_zero_duration() {
            common::interval::interval_zero_duration::<futures_timer::Delay>().await;
        }

        #[futures_test::test]
        async fn burst() {
            common::interval::burst::<futures_timer::Delay>().await;
        }

        #[futures_test::test]
        async fn delay() {
            common::interval::delay::<futures_timer::Delay>().await;
        }

        #[futures_test::test]
        async fn skip() {
            common::interval::skip::<futures_timer::Delay>().await;
        }

        #[futures_test::test]
        async fn reset() {
            common::interval::reset::<futures_timer::Delay>().await;
        }
    }
}