// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::builtin::H256;
#[allow(unused_imports)]
use crate::{env, types::*};

pub struct Ledger;

impl Ledger {
    #[inline(always)]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_ledger_contract_hash() }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0xda65b600f7124ce6c79950c1772a36403104f2be")
    }

    #[inline(always)]
    pub fn current_block_index() -> u32 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_ledger_current_block_index() }

        #[cfg(not(target_family = "wasm"))]
        0
    }

    #[inline(always)]
    pub fn current_block_hash() -> H256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_ledger_current_block_hash() }
        #[cfg(not(target_family = "wasm"))]
        H256::zero()
    }

    #[inline(always)]
    pub fn block_of_index(
        #[cfg_attr(not(target_family = "wasm"), allow(unused_variables))] index: u32,
    ) -> Block {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_ledger_block_of_index(index) }
        #[cfg(not(target_family = "wasm"))]
        Block::default()
    }

    #[inline(always)]
    pub fn block_of_hash(
        #[cfg_attr(not(target_family = "wasm"), allow(unused_variables))] hash: H256,
    ) -> Block {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_ledger_block_of_hash(hash) }
        #[cfg(not(target_family = "wasm"))]
        Block::default()
    }
}
