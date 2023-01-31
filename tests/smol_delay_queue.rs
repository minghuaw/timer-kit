#[macro_use]
mod macros;

mod common;

cfg_smol! {
    #[smol_potat::test]
    async fn single_immediate_delay() {
        common::delay_queue::single_immediate_delay::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn multiple_immediate_delay() {
        common::delay_queue::multiple_immediate_delay::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn single_short_delay() {
        common::delay_queue::single_short_delay::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn multi_delay_at_start() {
        common::delay_queue::multi_delay_at_start::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn insert_in_past_fires_immediately() {
        common::delay_queue::insert_in_past_fires_immediately::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn remove_entry() {
        common::delay_queue::remove_entry::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn reset_entry() {
        common::delay_queue::reset_entry::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn reset_much_later() {
        common::delay_queue::reset_much_later::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn reset_twice() {
        common::delay_queue::reset_twice::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn repeatedly_reset_entry_inserted_as_expired() {
        common::delay_queue::repeatedly_reset_entry_inserted_as_expired::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn remove_expired_item() {
        common::delay_queue::remove_expired_item::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn remove_at_timer_wheel_threshold() {
        common::delay_queue::remove_at_timer_wheel_threshold::<smol::Timer>().await;
    }

    #[smol_potat::test]
    async fn expires_before_last_insert() {
        common::delay_queue::expires_before_last_insert::<smol::Timer>().await;
    }
}