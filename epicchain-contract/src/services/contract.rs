// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[cfg(target_family = "wasm")]
use crate::{env, types::{Any, Array, ByteString, CallFlags, H160, PublicKey}};

#[cfg(not(target_family = "wasm"))]
use crate::types::{Any, Array, ByteString, CallFlags, H160, PublicKey};

/// Provides functionality for interacting with contracts.
pub struct Contract;

#[cfg(not(target_family = "wasm"))]
impl Contract {
    /// Calls a method on another contract.
    #[inline(always)]
    #[allow(unused_variables)]
    pub fn call(_contract: H160, _method: ByteString, _call_flags: CallFlags, _args: Array<Any>) -> Any {
        // For non-WASM targets (tests), return default Any
        Any::default()
    }

    /// Gets the call flags of the current execution context.
    #[inline(always)]
    pub fn get_call_flags() -> CallFlags {
        // For non-WASM targets (tests), return default call flags
        CallFlags::All
    }

    /// Creates a standard account with the given public key.
    #[inline(always)]
    #[allow(unused_variables)]
    pub fn create_standard_account(_public_key: PublicKey) -> H160 {
        // For non-WASM targets (tests), return a mock non-zero hash
        H160::from_bytes(&[1u8; 20])
    }

    /// Creates a multi-signature account with the given public keys.
    #[inline(always)]
    #[allow(unused_variables)]
    pub fn create_multisig_account(_min_signers: u32, _public_keys: Array<PublicKey>) -> H160 {
        // For non-WASM targets (tests), return a mock non-zero hash
        H160::from_bytes(&[2u8; 20])
    }
}

#[cfg(target_family = "wasm")]
impl Contract {
    /// Calls a method on another contract.
    #[inline(always)]
    pub fn call(contract: H160, method: ByteString, call_flags: CallFlags, args: Array<Any>) -> Any {
        unsafe { env::syscall::system_contract_call(contract, method, call_flags, args) }
    }

    /// Gets the call flags of the current execution context.
    #[inline(always)]
    pub fn get_call_flags() -> CallFlags {
        unsafe { env::syscall::system_contract_get_call_flags() }
    }

    /// Creates a standard account with the given public key.
    #[inline(always)]
    pub fn create_standard_account(public_key: PublicKey) -> H160 {
        unsafe { env::syscall::system_contract_create_standard_account(public_key) }
    }

    /// Creates a multi-signature account with the given public keys.
    #[inline(always)]
    pub fn create_multisig_account(min_signers: u32, public_keys: Array<PublicKey>) -> H160 {
        unsafe { env::syscall::system_contract_create_multi_signs_account(min_signers, public_keys) }
    }
}
