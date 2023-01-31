#[macro_use]
mod macros;

mod util;

cfg_smol! {
    #[smol_potat::test]
    async fn immediate_sleep() {
        util::sleep::immediate_sleep::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn short_sleep() {
        util::sleep::short_sleep::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn reset() {
        util::sleep::reset::<smol::Timer>().await;
    }
}