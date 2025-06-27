// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

//! Module for macro-related definitions

mod safe;
pub use safe::safe;

/// Macro to define a function that is only available in WASM target
#[macro_export]
macro_rules! wasm_func {
    ($(#[$attr:meta])* pub fn $name:ident($($arg:ident: $type:ty),*) -> $ret:ty $body:block) => {
        #[cfg(target_family = "wasm")]
        $(#[$attr])*
        pub fn $name($($arg: $type),*) -> $ret $body

        #[cfg(not(target_family = "wasm"))]
        #[allow(unused_variables)]
        $(#[$attr])*
        pub fn $name($($arg: $type),*) -> $ret {
            Default::default()
        }
    };
    ($(#[$attr:meta])* pub fn $name:ident($($arg:ident: $type:ty),*) $body:block) => {
        #[cfg(target_family = "wasm")]
        $(#[$attr])*
        pub fn $name($($arg: $type),*) $body

        #[cfg(not(target_family = "wasm"))]
        #[allow(unused_variables)]
        $(#[$attr])*
        pub fn $name($($arg: $type),*) {
            // Non-WASM implementation - no operation
        }
    };
}
