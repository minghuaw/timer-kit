cfg_not_wasm32! {
    cfg_tokio! {
        mod tokio;

        pub type DefaultDelay = tokio::time::Sleep;
    }

    cfg_futures_timer! {
        mod futures_timer;

        pub type DefaultDelay = futures_timer::Delay;
    }
}

cfg_wasm32! {
    mod wasm;

    pub type DefaultDelay = wasm_timer::Delay;
}