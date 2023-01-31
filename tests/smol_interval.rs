#[macro_use]
mod macros;

cfg_smol! {
    mod util;

    #[smol_potat::test]
    #[should_panic]
    async fn interval_zero_duration() {
        util::interval_zero_duration::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn burst() {
        util::burst::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn delay() {
        util::delay::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn skip() {
        util::skip::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn reset() {
        util::reset::<smol::Timer>().await;
    }
}