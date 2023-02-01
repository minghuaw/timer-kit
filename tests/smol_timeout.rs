#[macro_use]
mod macros;

mod common;

cfg_not_wasm32! {
    cfg_smol! {
        #[smol_potat::test]
        async fn simultaneous_deadline_future_completion() {
            common::timeout::simultaneous_deadline_future_completion::<smol::Timer>().await;
        }
    
        #[smol_potat::test]
        async fn completed_future_past_deadline() {
            common::timeout::completed_future_past_deadline::<smol::Timer>().await;
        }
    
        #[smol_potat::test]
        async fn future_and_deadline_in_future() {
            common::timeout::future_and_deadline_in_future::<smol::Timer>().await;
        }
    
        #[smol_potat::test]
        async fn deadline_future_elapses() {
            common::timeout::deadline_future_elapses::<smol::Timer>().await;
        }
    }
}
