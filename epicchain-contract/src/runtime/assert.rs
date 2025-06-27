// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::ByteString;

#[inline(always)]
pub fn assert(condition: bool) {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::asm::assert(condition) };

    #[cfg(not(target_family = "wasm"))]
    assert!(condition);
}

#[inline(always)]
pub fn assert_with_message(condition: bool, message: ByteString) {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::asm::assert_with_message(condition, message) };

    #[cfg(not(target_family = "wasm"))]
    assert!(condition, "{}", message.to_string());
}

#[inline(always)]
pub fn abort() {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::asm::abort() };

    #[cfg(not(target_family = "wasm"))]
    panic!();
}

#[inline(always)]
pub fn abort_with_message(message: ByteString) {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::asm::abort_with_message(message) };

    #[cfg(not(target_family = "wasm"))]
    panic!("{}", message.to_string());
}
