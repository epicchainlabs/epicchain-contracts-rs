// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};

pub struct Policy;

impl Policy {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_policy_contract_hash() }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0xcc5e4edd9f5f8dba8bb65734541df7a1c081c67b")
    }

    /// Gets the fee per byte for transactions
    #[inline(always)]
    #[rustfmt::skip]
    pub fn get_fee_per_byte() -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_policy_get_fee_per_byte() }

        #[cfg(not(target_family = "wasm"))]
        Int256::new(1000) // Mock: 1000 GAS fractions per byte
    }

    /// Gets the execution fee factor
    #[inline(always)]
    #[rustfmt::skip]
    pub fn get_exec_fee_factor() -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_policy_get_exec_fee_factor() }

        #[cfg(not(target_family = "wasm"))]
        Int256::new(30) // Mock: 30x execution fee factor
    }

    /// Gets the storage price
    #[inline(always)]
    #[rustfmt::skip]
    pub fn get_storage_price() -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_policy_get_storage_price() }

        #[cfg(not(target_family = "wasm"))]
        Int256::new(100000) // Mock: 100000 GAS fractions per storage byte
    }

    /// Checks if an account is blocked
    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn is_blocked(account: H160) -> bool {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_policy_is_blocked(account) }

        #[cfg(not(target_family = "wasm"))]
        false // Mock: no accounts blocked
    }

    /// Gets the attribute fee for a specific transaction attribute type
    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn get_attribute_fee(attr_type: consts::TxAttrType) -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_policy_get_attr_fee(attr_type) }

        #[cfg(not(target_family = "wasm"))]
        Int256::zero() // Mock: no attribute fees
    }

    /// Sets the attribute fee for a specific transaction attribute type
    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn set_attribute_fee(attr_type: consts::TxAttrType, fee: Int256) {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_policy_set_attr_fee(attr_type, fee) }

        #[cfg(not(target_family = "wasm"))]
        {} // Mock: no-op
    }
}
