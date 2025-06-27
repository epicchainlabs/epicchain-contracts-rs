// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{contract::*, env, types::*};

pub struct Gas;

impl Gas {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicpulse_contract_hash() }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0xd2a4cff31913016155e38e474a2c06d08be276cf")
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn symbol() -> ByteString {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicpulse_symbol() }

        #[cfg(not(target_family = "wasm"))]
        ByteString::new("GAS".as_bytes().to_vec())
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn total_supply() -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicpulse_total_supply() }

        #[cfg(not(target_family = "wasm"))]
        Int256::new(100_000_000_00000000i64) // 100M GAS with 8 decimals
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn decimals() -> u32 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicpulse_decimals() }

        #[cfg(not(target_family = "wasm"))]
        8
    }

    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn balance_of(account: H160) -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicpulse_balance_of(account) }

        #[cfg(not(target_family = "wasm"))]
        Int256::new(0) // Mock implementation for non-WASM targets
    }

    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicpulse_transfer(from, to, amount) }

        #[cfg(not(target_family = "wasm"))]
        false // Mock implementation for non-WASM targets
    }
}
