// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![allow(unused)]

#[cfg(target_family = "wasm")]
use crate::types::{Any, ByteString, Contract, H160, Int256};

/// Native contract method declarations for EpicChain blockchain
#[link(wasm_import_module = "epicchain.native")]
#[cfg(target_family = "wasm")]
extern "C" {
    // EpicChain native methods
    pub(crate) fn native_epicchain_transfer(from: H160, to: H160, amount: Int256) -> bool;
    pub(crate) fn native_epicchain_get_balance(account: H160) -> Int256;
    pub(crate) fn native_epicchain_get_name() -> ByteString;
    pub(crate) fn native_epicchain_get_symbol() -> ByteString;
    pub(crate) fn native_epicchain_get_decimals() -> u32;
    pub(crate) fn native_epicchain_get_total_supply() -> Int256;

    // EpicPulse native methods
    pub(crate) fn native_epicpulse_transfer(from: H160, to: H160, amount: Int256) -> bool;
    pub(crate) fn native_epicpulse_get_balance(account: H160) -> Int256;
    pub(crate) fn native_epicpulse_get_name() -> ByteString;
    pub(crate) fn native_epicpulse_get_symbol() -> ByteString;
    pub(crate) fn native_epicpulse_get_decimals() -> u32;
    pub(crate) fn native_epicpulse_get_total_supply() -> Int256;

    // EssentialLib native methods
    pub(crate) fn native_essentiallib_base64_encode(data: ByteString) -> ByteString;
    pub(crate) fn native_essentiallib_base64_decode(data: ByteString) -> ByteString;
    pub(crate) fn native_essentiallib_json_serialize(item: Any) -> ByteString;
    pub(crate) fn native_essentiallib_json_deserialize(json: ByteString) -> Any;
    pub(crate) fn native_essentiallib_itoa(value: Int256) -> ByteString;
    pub(crate) fn native_essentiallib_atoi(value: ByteString) -> Int256;

    // ContractManagement native methods
    pub(crate) fn native_contract_get_contract(script_hash: H160) -> Contract;
    pub(crate) fn native_contract_deploy(nef_file: ByteString, manifest: ByteString, data: Any) -> Contract;
    pub(crate) fn native_contract_update(nef_file: ByteString, manifest: ByteString, data: Any) -> bool;
    pub(crate) fn native_contract_destroy() -> bool;
}