// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

/// Encode binary data to base64 string
#[inline(always)]
pub fn base64_encode(_data: ByteString) -> ByteString {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::native::native_essentiallib_base64_encode(_data) }

    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        ByteString::empty()
    }
}

/// Decode base64 string to binary data
#[inline(always)]
pub fn base64_decode(_data: ByteString) -> ByteString {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::native::native_essentiallib_base64_decode(_data) }

    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        ByteString::empty()
    }
}

/// Serialize an object to JSON string
#[inline(always)]
pub fn json_serialize(_item: Any) -> ByteString {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::native::native_essentiallib_json_serialize(_item) }

    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        ByteString::empty()
    }
}

/// Deserialize a JSON string to an object
#[inline(always)]
pub fn json_deserialize(_json: ByteString) -> Any {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::native::native_essentiallib_json_deserialize(_json) }

    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        Any::default()
    }
}

/// Convert an integer to a string
#[inline(always)]
pub fn itoa(_value: Int256) -> ByteString {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::native::native_essentiallib_itoa(_value) }

    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        ByteString::empty()
    }
}

/// Convert a string to an integer
#[inline(always)]
pub fn atoi(_value: ByteString) -> Int256 {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::native::native_essentiallib_atoi(_value) }

    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        Int256::zero()
    }
}