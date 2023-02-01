#[macro_use]
mod macros;

mod common;

cfg_not_wasm32! {
    cfg_tokio! {
        #[tokio::test]
        async fn single_immediate_delay() {
            common::delay_queue::single_immediate_delay::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn multiple_immediate_delay() {
            common::delay_queue::multiple_immediate_delay::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn single_short_delay() {
            common::delay_queue::single_short_delay::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn multi_delay_at_start() {
            common::delay_queue::multi_delay_at_start::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn insert_in_past_fires_immediately() {
            common::delay_queue::insert_in_past_fires_immediately::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn remove_entry() {
            common::delay_queue::remove_entry::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn reset_entry() {
            common::delay_queue::reset_entry::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn reset_much_later() {
            common::delay_queue::reset_much_later::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn reset_twice() {
            common::delay_queue::reset_twice::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn repeatedly_reset_entry_inserted_as_expired() {
            common::delay_queue::repeatedly_reset_entry_inserted_as_expired::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn remove_expired_item() {
            common::delay_queue::remove_expired_item::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn remove_at_timer_wheel_threshold() {
            common::delay_queue::remove_at_timer_wheel_threshold::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn expires_before_last_insert() {
            common::delay_queue::expires_before_last_insert::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn multi_reset() {
            common::delay_queue::multi_reset::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn expire_first_key_when_reset_to_expire_earlier() {
            common::delay_queue::expire_first_key_when_reset_to_expire_earlier::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn expire_second_key_when_reset_to_expire_earlier() {
            common::delay_queue::expire_second_key_when_reset_to_expire_earlier::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn reset_first_expiring_item_to_expire_later() {
            common::delay_queue::reset_first_expiring_item_to_expire_later::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn insert_before_first_after_poll() {
            common::delay_queue::insert_before_first_after_poll::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn insert_after_ready_poll() {
            common::delay_queue::insert_after_ready_poll::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn reset_later_after_slot_starts() {
            common::delay_queue::reset_later_after_slot_starts::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn reset_insert_expired() {
            common::delay_queue::reset_insert_expired::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn reset_earlier_after_slot_starts() {
            common::delay_queue::reset_earlier_after_slot_starts::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn insert_in_past_after_poll_fires_immediately() {
            common::delay_queue::insert_in_past_after_poll_fires_immediately::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn delay_queue_poll_expired_when_empty() {
            common::delay_queue::delay_queue_poll_expired_when_empty::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn compact_expire_empty() {
            common::delay_queue::compact_expire_empty::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn compact_remove_empty() {
            common::delay_queue::compact_remove_empty::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn compact_remove_remapped_keys() {
            common::delay_queue::compact_remove_remapped_keys::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn compact_change_deadline() {
            common::delay_queue::compact_change_deadline::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        async fn remove_after_compact() {
            common::delay_queue::remove_after_compact::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        #[should_panic]
        async fn panic_on_remove_of_nonexistent_key() {
            common::delay_queue::panic_on_remove_of_nonexistent_key::<tokio::time::Sleep>().await;
        }
    
        #[tokio::test]
        #[should_panic]
        async fn panic_on_remove_after_compact_poll() {
            common::delay_queue::panic_on_remove_after_compact_poll::<tokio::time::Sleep>().await;
        }
    }
}

