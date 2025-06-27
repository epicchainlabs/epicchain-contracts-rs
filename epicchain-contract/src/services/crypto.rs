// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[cfg(target_family = "wasm")]
use crate::{env, types::{Array, ByteString, PublicKey}};

#[cfg(not(target_family = "wasm"))]
use crate::types::{Array, ByteString, PublicKey};

/// Provides cryptographic functionality.
pub struct Crypto;

#[cfg(not(target_family = "wasm"))]
impl Crypto {
    /// Verifies that the signature is valid for the given public key and message.
    #[inline(always)]
    #[allow(unused_variables)]
    pub fn check_signature(_public_key: PublicKey, _signature: ByteString) -> bool {
        // For non-WASM targets (tests), return false (dummy signatures are invalid)
        false
    }

    /// Verifies that the signatures are valid for the given public keys and message.
    #[inline(always)]
    #[allow(unused_variables)]
    pub fn check_multisig(_public_keys: Array<PublicKey>, _signatures: Array<ByteString>) -> bool {
        // For non-WASM targets (tests), return false (dummy signatures are invalid)
        false
    }
}

#[cfg(target_family = "wasm")]
impl Crypto {
    /// Verifies that the signature is valid for the given public key and message.
    #[inline(always)]
    pub fn check_signature(public_key: PublicKey, signature: ByteString) -> bool {
        unsafe { env::syscall::system_crypto_check_sign(public_key, signature) }
    }

    /// Verifies that the signatures are valid for the given public keys and message.
    #[inline(always)]
    pub fn check_multisig(public_keys: Array<PublicKey>, signatures: Array<ByteString>) -> bool {
        unsafe { env::syscall::system_crypto_check_multi_signs(public_keys, signatures) }
    }
}
