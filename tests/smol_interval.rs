#[macro_use]
mod macros;

mod common;

cfg_smol! {
    #[smol_potat::test]
    #[should_panic]
    async fn interval_zero_duration() {
        common::interval::interval_zero_duration::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn burst() {
        common::interval::burst::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn delay() {
        common::interval::delay::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn skip() {
        common::interval::skip::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn reset() {
        common::interval::reset::<smol::Timer>().await;
    }
}