#[macro_use]
mod macros;

mod interval;
mod sleep;
mod timeout;
mod delay_queue;

pub trait Delay { }

pub trait AsyncDelay { 
    
}