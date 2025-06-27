// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub(crate) mod hash;

pub use hash::*;

use crate::types::*;

#[cfg(target_family = "wasm")]
use crate::env;

#[inline(always)]
pub fn check_sign(_public_key: PublicKey, _sign: ByteString) -> bool {
    #[cfg(target_family = "wasm")]
    unsafe { env::syscall::system_crypto_check_sign(_public_key, _sign) }

    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        true
    }
}

#[inline(always)]
pub fn check_multi_signs(_public_keys: Array<PublicKey>, _signs: Array<ByteString>) -> bool {
    #[cfg(target_family = "wasm")]
    unsafe { env::syscall::system_crypto_check_multi_signs(_public_keys, _signs) }

    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        true
    }
}

#[inline(always)]
pub fn verify_ecdsa(
    _message: ByteString,
    _public_key: PublicKey,
    _sign: ByteString,
    _named_curve_hash: NamedCurveHash,
) -> bool {
    #[cfg(target_family = "wasm")]
    unsafe { env::crypto::verify_ecdsa(_message, _public_key, _sign, _named_curve_hash) }

    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        true
    }
}

#[inline(always)]
pub fn verify_ed25519(_message: ByteString, _public_key: PublicKey, _sign: ByteString) -> bool {
    #[cfg(target_family = "wasm")]
    unsafe { env::crypto::verify_ed25519(_message, _public_key, _sign) }

    #[cfg(not(target_family = "wasm"))]
    {
        // Mock implementation for non-WASM targets
        true
    }
}
