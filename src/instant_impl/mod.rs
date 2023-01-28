mod std;

cfg_not_wasm32! {
    cfg_tokio! {
        mod tokio;
    }
}