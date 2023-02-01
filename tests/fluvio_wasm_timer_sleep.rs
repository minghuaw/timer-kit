#[macro_use]
mod macros;

mod common;

cfg_wasm32! {
    cfg_fluvio_wasm_timer! {
        use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

        wasm_bindgen_test_configure!(run_in_browser);

        #[wasm_bindgen_test]
        async fn immediate_sleep() {
            common::sleep::immediate_sleep::<fluvio_wasm_timer::Delay>().await;
        }

        #[wasm_bindgen_test]
        async fn short_sleep() {
            common::sleep::short_sleep::<fluvio_wasm_timer::Delay>().await;
        }

        #[wasm_bindgen_test]
        async fn reset() {
            common::sleep::reset::<fluvio_wasm_timer::Delay>().await;
        }
    }
}