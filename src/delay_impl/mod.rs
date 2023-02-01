cfg_not_wasm32! {
    cfg_tokio! {
        mod tokio;
    }

    cfg_smol! {
        mod smol;
    }

    cfg_futures_timer! {
        mod futures_timer;
    }
}

cfg_wasm32! {
    cfg_wasm_timer! {
        mod wasm_timer;
    }

    cfg_fluvio_wasm_timer! {
        mod fluvio_wasm_timer;
    }
}