use std::task::{Context, Poll};

use error::Error;

#[macro_use]
mod macros;

mod interval;
mod sleep;
mod timeout;
mod delay_queue;

cfg_not_wasm32! {
    mod tokio;
    mod async_std;
    mod smol;
}

cfg_wasm32! {
    mod wasm;
}

pub mod error;

pub trait Delay { }

pub trait AsyncDelay { 
    fn poll_deadline(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Error>>;
}