#[macro_use]
mod macros;

mod util;

cfg_smol! {
    #[smol_potat::test]
    #[should_panic]
    async fn interval_zero_duration() {
        util::interval::interval_zero_duration::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn burst() {
        util::interval::burst::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn delay() {
        util::interval::delay::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn skip() {
        util::interval::skip::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn reset() {
        util::interval::reset::<smol::Timer>().await;
    }
}