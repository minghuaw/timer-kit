#[macro_use]
mod macros;

mod common;

cfg_not_wasm32! {
    cfg_tokio! {
        #[tokio::test]
        async fn simultaneous_deadline_future_completion() {
            common::timeout::simultaneous_deadline_future_completion::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn completed_future_past_deadline() {
            common::timeout::completed_future_past_deadline::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn future_and_deadline_in_future() {
            common::timeout::future_and_deadline_in_future::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn deadline_future_elapses() {
            common::timeout::deadline_future_elapses::<tokio::time::Sleep>().await;
        }
    
        // #[tokio::test]
        // async fn timeout_is_not_exhausted_by_future() {
        //     common::timeout::timeout_is_not_exhausted_by_future::<tokio::time::Sleep>().await;
        // }
    }
}
