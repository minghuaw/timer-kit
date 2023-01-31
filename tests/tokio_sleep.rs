#[macro_use]
mod macros;

cfg_tokio! {
    mod util;

    #[tokio::test]
    async fn immediate_sleep() {
        util::immediate_sleep::<tokio::time::Sleep>().await;
    }
}