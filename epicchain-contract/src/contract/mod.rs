// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub mod native;

pub(crate) mod event;
pub(crate) mod nep;
pub(crate) mod nep11;
pub(crate) mod nep17;
pub(crate) mod token;

pub use {event::*, nep::*, nep11::*, nep17::*};

use crate::types::*;

#[cfg(target_family = "wasm")]
use crate::env;

#[inline(always)]
pub fn call(
    #[cfg_attr(not(target_family = "wasm"), allow(unused_variables))] contract: H160,
    #[cfg_attr(not(target_family = "wasm"), allow(unused_variables))] method: ByteString,
    #[cfg_attr(not(target_family = "wasm"), allow(unused_variables))] call_flags: CallFlags,
    #[cfg_attr(not(target_family = "wasm"), allow(unused_variables))] args: Array<Any>,
) -> Any {
    #[cfg(target_family = "wasm")]
    unsafe { env::syscall::system_contract_call(contract, method, call_flags, args) }
    #[cfg(not(target_family = "wasm"))]
    Any::default()
}

#[inline(always)]
pub fn get_call_flags() -> CallFlags {
    #[cfg(target_family = "wasm")]
    unsafe { env::syscall::system_contract_get_call_flags() }
    #[cfg(not(target_family = "wasm"))]
    CallFlags::None
}

#[inline(always)]
pub fn create_standard_account(
    #[cfg_attr(not(target_family = "wasm"), allow(unused_variables))] public_key: PublicKey,
) -> H160 {
    #[cfg(target_family = "wasm")]
    unsafe { env::syscall::system_contract_create_standard_account(public_key) }
    #[cfg(not(target_family = "wasm"))]
    H160::zero()
}

#[inline(always)]
pub fn create_multi_signs_account(
    #[cfg_attr(not(target_family = "wasm"), allow(unused_variables))] min_signers: u32,
    #[cfg_attr(not(target_family = "wasm"), allow(unused_variables))] public_keys: Array<PublicKey>,
) -> H160 {
    #[cfg(target_family = "wasm")]
    unsafe { env::syscall::system_contract_create_multi_signs_account(min_signers, public_keys) }
    #[cfg(not(target_family = "wasm"))]
    H160::zero()
}
