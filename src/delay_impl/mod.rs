cfg_not_wasm32! {
    cfg_tokio! {
        mod tokio;
    }

    cfg_smol! {
        mod smol;
    }
}

cfg_wasm32! {
    cfg_wasm_timer! {
        mod wasm_timer;
    }
}