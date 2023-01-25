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

macro_rules! cfg_async_std {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "async-std")]
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

macro_rules! cfg_smol {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "smol")]
            $item
        )*
    };
}