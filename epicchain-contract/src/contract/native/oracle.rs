// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};

pub struct Oracle;

impl Oracle {
    /// Minimum response fee constant
    pub const MINIMUM_RESPONSE_FEE: u64 = 10_000_000; // 0.1 GAS

    #[inline(always)]
    #[rustfmt::skip]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_oracle_contract_hash() }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0xfe924b7cfe89ddd271abaf7210a80a7e11178758")
    }

    /// Gets the current oracle price
    #[inline(always)]
    #[rustfmt::skip]
    pub fn get_price() -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_oracle_get_price() }

        #[cfg(not(target_family = "wasm"))]
        Int256::new(50_000_000) // Mock: 0.5 XPP
    }

    /// Requests data from an oracle
    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn request(
        url: ByteString,
        filter: ByteString,
        callback: ByteString,
        user_data: Any,
        epicpulse_for_response: Int256,
    ) -> bool {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_oracle_response(url, filter, callback, user_data, epicpulse_for_response) }

        #[cfg(not(target_family = "wasm"))]
        true // Mock: always successful
    }
}
