// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[cfg(target_family = "wasm")]
use crate::{env, types::{Array, Any, ByteString}};

#[cfg(not(target_family = "wasm"))]
use crate::types::{Array, Any, ByteString};

/// Provides functionality for emitting events.
pub struct Event;

#[cfg(not(target_family = "wasm"))]
impl Event {
    /// Emits an event with the given name and state.
    #[inline(always)]
    #[allow(unused_variables)]
    pub fn emit(_name: ByteString, _state: Array<Any>) {
        unimplemented!("This function is only available in WASM target")
    }
}

#[cfg(target_family = "wasm")]
impl Event {
    /// Emits an event with the given name and state.
    #[inline(always)]
    pub fn emit(name: ByteString, state: Array<Any>) {
        unsafe { env::syscall::system_runtime_notify(name, state) }
    }
}
