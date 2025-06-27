// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[repr(C)]
#[derive(Default)]
pub struct Tx {
    hash: H256,
    version: u32,
    nonce: u32,
    sender: H160,
    system_fee: Int256,
    network_fee: Int256,
    valid_until_block: u32,
    script: ByteString,
}

/// all getters for Tx
impl Tx {
    #[inline(always)]
    pub fn hash(&self) -> H256 {
        self.hash
    }

    #[inline(always)]
    pub fn version(&self) -> u32 {
        self.version
    }

    #[inline(always)]
    pub fn nonce(&self) -> u32 {
        self.nonce
    }

    #[inline(always)]
    pub fn sender(&self) -> H160 {
        self.sender
    }

    #[inline(always)]
    pub fn system_fee(&self) -> Int256 {
        self.system_fee
    }

    #[inline(always)]
    pub fn network_fee(&self) -> Int256 {
        self.network_fee
    }

    #[inline(always)]
    pub fn valid_until_block(&self) -> u32 {
        self.valid_until_block
    }

    /// Returns a clone of the script to avoid reference issues in wasm-to-EpicChain compilation
    #[inline(always)]
    pub fn script(&self) -> ByteString {
        self.script.clone()
    }
}
