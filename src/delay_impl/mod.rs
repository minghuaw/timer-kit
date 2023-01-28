cfg_not_wasm32! {
    cfg_tokio! {
        mod tokio;
    }

    cfg_futures_timer! {
        mod futures_timer;
    }
}

cfg_wasm32! {
    cfg_wasm_timer! {
        mod wasm_timer;
    }
}