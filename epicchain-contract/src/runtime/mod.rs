// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub mod assert;

pub use assert::*;

use crate::{types::*, wasm_func};

#[cfg(target_family = "wasm")]
use crate::env;

wasm_func! {
    #[inline(always)]
    pub fn get_trigger() -> TriggerType {
        unsafe { env::syscall::system_runtime_trigger() }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn get_platform() -> ByteString {
        unsafe { env::syscall::system_runtime_platform() }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn get_tx() -> Tx {
        unsafe { env::syscall::system_runtime_tx() }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn get_executing_script_hash() -> H160 {
        unsafe { env::syscall::system_runtime_executing_script_hash() }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn get_calling_script_hash() -> H160 {
        unsafe { env::syscall::system_runtime_calling_script_hash() }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn get_entry_script_hash() -> H160 {
        unsafe { env::syscall::system_runtime_entry_script_hash() }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn get_time() -> u64 {
        unsafe { env::syscall::system_runtime_time() }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn get_invocation_counter() -> u32 {
        unsafe { env::syscall::system_runtime_invocation_counter() }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn get_epicpulse_left() -> Int256 {
        unsafe { env::syscall::system_runtime_epicpulse_left() }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn get_address_version() -> u32 {
        unsafe { env::syscall::system_runtime_address_version() }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn get_notifications() -> Array<Notification> {
        unsafe { env::syscall::system_runtime_notifications() }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn get_notifications_by_hash(script_hash: H160) -> Array<Notification> {
        unsafe { env::syscall::system_runtime_get_notifications(script_hash) }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn check_witness_with_account(account: H160) -> bool {
        unsafe { env::syscall::system_runtime_check_witness_with_account(account) }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn check_witness_with_public_key(public_key: PublicKey) -> bool {
        unsafe { env::syscall::system_runtime_check_witness_with_public_key(public_key) }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn log(message: ByteString) {
        unsafe { env::syscall::system_runtime_log(message) }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn burn_epicpulse(amount: Int256) {
        unsafe { env::syscall::system_runtime_burn_epicpulse(amount) }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn get_random() -> Int256 {
        unsafe { env::syscall::system_runtime_get_random() }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn get_network() -> u32 {
        unsafe { env::syscall::system_runtime_get_network() }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn load_script(script_hash: H160, call_flags: CallFlags, args: Array<Any>) -> Any {
        unsafe { env::syscall::system_runtime_load_script(script_hash, call_flags, args) }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn current_signers() -> Array<Signer> {
        unsafe { env::syscall::system_runtime_current_signers() }
    }
}

wasm_func! {
    #[inline(always)]
    pub fn notify(name: ByteString, data: Array<Any>) {
        unsafe { env::syscall::system_runtime_notify(name, data) }
    }
}
