// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::{types::*};

/// Transfer GAS tokens from one account to another
#[inline(always)]
pub fn transfer(_from: H160, _to: H160, _amount: Int256) -> bool {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::native::native_epicpulse_transfer(_from, _to, _amount) }
    
    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        true
    }
}

/// Get the GAS balance of an account
#[inline(always)]
pub fn get_balance(_account: H160) -> Int256 {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::native::native_epicpulse_get_balance(_account) }
    
    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        Int256::zero()
    }
}

/// Get the name of the GAS token
#[inline(always)]
pub fn get_name() -> ByteString {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::native::native_epicpulse_get_name() }
    
    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        ByteString::from_literal("GAS")
    }
}

/// Get the symbol of the GAS token
#[inline(always)]
pub fn get_symbol() -> ByteString {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::native::native_epicpulse_get_symbol() }
    
    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        ByteString::from_literal("GAS")
    }
}

/// Get the decimal precision of the GAS token
#[inline(always)]
pub fn get_decimals() -> u32 {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::native::native_epicpulse_get_decimals() }
    
    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        8
    }
}

/// Get the total supply of GAS tokens
#[inline(always)]
pub fn get_total_supply() -> Int256 {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::native::native_epicpulse_get_total_supply() }
    
    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        Int256::zero()
    }
}
