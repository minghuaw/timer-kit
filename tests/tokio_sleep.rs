#[macro_use]
mod macros;

mod common;

cfg_not_wasm32! {
    cfg_tokio! {
        #[tokio::test]
        async fn immediate_sleep() {
            common::sleep::immediate_sleep::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn short_sleep() {
            common::sleep::short_sleep::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn reset() {
            common::sleep::reset::<tokio::time::Sleep>().await;
        }
    }
}
