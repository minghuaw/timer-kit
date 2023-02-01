#[macro_use]
mod macros;

mod common;

cfg_not_wasm32! {
    cfg_tokio! {
        #[tokio::test]
        #[should_panic]
        async fn interval_zero_duration() {
            common::interval::interval_zero_duration::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn burst() {
            common::interval::burst::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn delay() {
            common::interval::delay::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn skip() {
            common::interval::skip::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn reset() {
            common::interval::reset::<tokio::time::Sleep>().await;
        }
    }
}
