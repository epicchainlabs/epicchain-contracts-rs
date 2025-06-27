// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub mod gas;
pub mod legder;
pub mod neo;
pub mod oracle;
pub mod policy;

pub use {gas::*, legder::*, epicchain::*, oracle::*, policy::*};

use crate::types::epicchain::Role;
#[cfg(target_family = "wasm")]
use crate::{env, types::*};

#[cfg(not(target_family = "wasm"))]
use crate::types::*;

pub struct ContractManagement;

impl ContractManagement {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_contract_management_contract_hash() }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0xfffdc93764dbaddd97c48f252a53ea4643faa3fd")
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn get_min_deployment_fee() -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_contract_management_get_min_deployment_fee() }

        #[cfg(not(target_family = "wasm"))]
        Int256::new(10_00000000) // 10 GAS with 8 decimals
    }

    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn contract_of_hash(hash: H160) -> Contract {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_contract_management_contract_of_hash(hash) }

        #[cfg(not(target_family = "wasm"))]
        Contract::default() // Mock implementation for non-WASM targets
    }

    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn contract_of_id(id: u32) -> Contract {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_contract_management_contract_of_id(id) }

        #[cfg(not(target_family = "wasm"))]
        Contract::default() // Mock implementation for non-WASM targets
    }

    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn has_method(hash: H160, method: ByteString, param_count: u32) -> bool {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_contract_management_has_method(hash, method, param_count) }

        #[cfg(not(target_family = "wasm"))]
        false // Mock implementation for non-WASM targets
    }

    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn deploy(nef: ByteString, manifest: ByteString) -> Contract {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_contract_management_deploy(nef, manifest) }

        #[cfg(not(target_family = "wasm"))]
        Contract::default() // Mock implementation for non-WASM targets
    }

    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn update(nef: ByteString, manifest: ByteString) {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_contract_management_update(nef, manifest) }

        #[cfg(not(target_family = "wasm"))]
        () // Mock implementation for non-WASM targets
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn destroy() {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_contract_management_destroy() }

        #[cfg(not(target_family = "wasm"))]
        () // Mock implementation for non-WASM targets
    }
}

pub struct RoleManagement;

impl RoleManagement {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_role_management_contract_hash() }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0x49cf4e5378ffcd4dec034fd98a174c5491e395e2")
    }

    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn get_designated_by_role(role: Role, block_index: u32) -> Array<PublicKey> {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_role_management_get_designated_by_role(role, block_index) }

        #[cfg(not(target_family = "wasm"))]
        Array::new() // Mock implementation for non-WASM targets
    }
}
