#[macro_use]
mod macros;

mod common;

cfg_smol! {
    #[smol_potat::test]
    async fn immediate_sleep() {
        common::sleep::immediate_sleep::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn short_sleep() {
        common::sleep::short_sleep::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn reset() {
        common::sleep::reset::<smol::Timer>().await;
    }
}