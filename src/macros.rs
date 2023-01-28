#![allow(unused_macros)]

macro_rules! cfg_wasm32 {
    ($($item:item)*) => {
        $(
            #[cfg(target_arch = "wasm32")]
            $item
        )*
    };
}

macro_rules! cfg_not_wasm32 {
    ($($item:item)*) => {
        $(
            #[cfg(not(target_arch = "wasm32"))]
            $item
        )*
    }
}

macro_rules! cfg_futures_timer {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "futures-timer")]
            $item
        )*
    };
}

macro_rules! cfg_tokio {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "tokio")]
            $item
        )*
    };
}

macro_rules! cfg_wasm_timer {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "wasm-timer")]
            $item
        )*
    };
}
