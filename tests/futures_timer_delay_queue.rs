#[macro_use]
mod macros;

mod common;

cfg_not_wasm32! {
    cfg_futures_timer! {
        #[futures_test::test]
        async fn single_immediate_delay() {
            common::delay_queue::single_immediate_delay::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn multiple_immediate_delay() {
            common::delay_queue::multiple_immediate_delay::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn single_short_delay() {
            common::delay_queue::single_short_delay::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn multi_delay_at_start() {
            common::delay_queue::multi_delay_at_start::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn insert_in_past_fires_immediately() {
            common::delay_queue::insert_in_past_fires_immediately::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn remove_entry() {
            common::delay_queue::remove_entry::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn reset_entry() {
            common::delay_queue::reset_entry::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn reset_much_later() {
            common::delay_queue::reset_much_later::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn reset_twice() {
            common::delay_queue::reset_twice::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn repeatedly_reset_entry_inserted_as_expired() {
            common::delay_queue::repeatedly_reset_entry_inserted_as_expired::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn remove_expired_item() {
            common::delay_queue::remove_expired_item::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn remove_at_timer_wheel_threshold() {
            common::delay_queue::remove_at_timer_wheel_threshold::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn expires_before_last_insert() {
            common::delay_queue::expires_before_last_insert::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn multi_reset() {
            common::delay_queue::multi_reset::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn expire_first_key_when_reset_to_expire_earlier() {
            common::delay_queue::expire_first_key_when_reset_to_expire_earlier::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn expire_second_key_when_reset_to_expire_earlier() {
            common::delay_queue::expire_second_key_when_reset_to_expire_earlier::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn reset_first_expiring_item_to_expire_later() {
            common::delay_queue::reset_first_expiring_item_to_expire_later::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn insert_before_first_after_poll() {
            common::delay_queue::insert_before_first_after_poll::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn insert_after_ready_poll() {
            common::delay_queue::insert_after_ready_poll::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn reset_later_after_slot_starts() {
            common::delay_queue::reset_later_after_slot_starts::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn reset_insert_expired() {
            common::delay_queue::reset_insert_expired::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn reset_earlier_after_slot_starts() {
            common::delay_queue::reset_earlier_after_slot_starts::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn insert_in_past_after_poll_fires_immediately() {
            common::delay_queue::insert_in_past_after_poll_fires_immediately::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn delay_queue_poll_expired_when_empty() {
            common::delay_queue::delay_queue_poll_expired_when_empty::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn compact_expire_empty() {
            common::delay_queue::compact_expire_empty::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn compact_remove_empty() {
            common::delay_queue::compact_remove_empty::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn compact_remove_remapped_keys() {
            common::delay_queue::compact_remove_remapped_keys::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn compact_change_deadline() {
            common::delay_queue::compact_change_deadline::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        async fn remove_after_compact() {
            common::delay_queue::remove_after_compact::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        #[should_panic]
        async fn panic_on_remove_of_nonexistent_key() {
            common::delay_queue::panic_on_remove_of_nonexistent_key::<futures_timer::Delay>().await;
        }
    
        #[futures_test::test]
        #[should_panic]
        async fn panic_on_remove_after_compact_poll() {
            common::delay_queue::panic_on_remove_after_compact_poll::<futures_timer::Delay>().await;
        }
    }
}