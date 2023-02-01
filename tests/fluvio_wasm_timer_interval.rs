//! All tests for the `fluvio-wasm-timer` crate.
//! 
//! These

#[macro_use]
mod macros;

mod common;

cfg_wasm32! {
    cfg_fluvio_wasm_timer! {
        use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

        wasm_bindgen_test_configure!(run_in_browser);

        // =====================================================================
        // interval
        // =====================================================================

        // #[wasm_bindgen_test]
        // #[should_panic] // TODO: this doesn't work in wasm
        // async fn interval_zero_duration() {
        //     common::interval::interval_zero_duration::<fluvio_wasm_timer::Delay>().await;
        // }

        #[wasm_bindgen_test]
        async fn burst() {
            common::interval::burst::<fluvio_wasm_timer::Delay>().await;
        }

        #[wasm_bindgen_test]
        async fn delay() {
            common::interval::delay::<fluvio_wasm_timer::Delay>().await;
        }

        #[wasm_bindgen_test]
        async fn skip() {
            common::interval::skip::<fluvio_wasm_timer::Delay>().await;
        }

        #[wasm_bindgen_test]
        async fn reset() {
            common::interval::reset::<fluvio_wasm_timer::Delay>().await;
        }
    }
}