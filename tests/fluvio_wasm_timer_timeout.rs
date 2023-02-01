#[macro_use]
mod macros;

mod common;

cfg_wasm32! {
    cfg_fluvio_wasm_timer! {
        use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

        wasm_bindgen_test_configure!(run_in_browser);

        #[wasm_bindgen_test]
        async fn simultaneous_deadline_future_completion() {
            common::timeout::simultaneous_deadline_future_completion::<fluvio_wasm_timer::Delay>().await;
        }

        #[wasm_bindgen_test]
        async fn completed_future_past_deadline() {
            common::timeout::completed_future_past_deadline::<fluvio_wasm_timer::Delay>().await;
        }

        #[wasm_bindgen_test]
        async fn future_and_deadline_in_future() {
            common::timeout::future_and_deadline_in_future::<fluvio_wasm_timer::Delay>().await;
        }

        #[wasm_bindgen_test]
        async fn deadline_future_elapses() {
            common::timeout::deadline_future_elapses::<fluvio_wasm_timer::Delay>().await;
        }
    }
}