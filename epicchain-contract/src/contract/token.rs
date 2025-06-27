// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::contract::{PREFIX_BALANCE, TOTAL_SUPPLY_KEY};

#[allow(unused_imports)]
use crate::{
    env,
    storage::StorageMap,
    types::{
        builtin::{
            h160::H160,
            int256::Int256,
            string::{ByteString, IntoByteString, FromByteString},
        },
    },
};

pub(crate) fn total_supply() -> Int256 {
    #[cfg(target_family = "wasm")]
    let key = unsafe { env::extension::concat_u8_byte_string(TOTAL_SUPPLY_KEY, ByteString::empty()) };

    #[cfg(not(target_family = "wasm"))]
    let key = ByteString::with_bytes(&[TOTAL_SUPPLY_KEY]);

    let storage = StorageMap::new();
    let value = storage.get(key.clone());
    if value.is_null() {
        Int256::zero()
    } else {
        Int256::from_byte_string(value.unwrap())
    }
}

pub(crate) fn balance_of(account: H160) -> Int256 {
    #[cfg(target_family = "wasm")]
    let key = unsafe { env::extension::concat_u8_byte_string(PREFIX_BALANCE, account.into_byte_string()) };

    #[cfg(not(target_family = "wasm"))]
    let key = ByteString::with_bytes(&[PREFIX_BALANCE]).concat(&account.into_byte_string());

    let storage = StorageMap::new();
    let value = storage.get(key.clone());
    if value.is_null() {
        Int256::zero()
    } else {
        Int256::from_byte_string(value.unwrap())
    }
}

// It must be inline becaue it has a reference argument.
// Otherwise, the compiled wasm ops cannot transfer to EpicChain ops.
#[inline(always)]
pub(crate) fn update_balance<const PREFIX: u8>(storage: &mut StorageMap, account: H160, amount: Int256) -> bool {
    #[cfg(target_family = "wasm")]
    let key = unsafe { env::extension::concat_u8_byte_string(PREFIX, account.into_byte_string()) };

    #[cfg(not(target_family = "wasm"))]
    let key = ByteString::with_bytes(&[PREFIX]).concat(&account.into_byte_string());

    let value = storage.get(key.clone());
    let balance = if value.is_null() {
        Int256::zero()
    } else {
        Int256::from_byte_string(value.unwrap())
    };

    let new_balance = balance.checked_add(&amount);
    if new_balance.is_negative() {
        return false;
    }

    if new_balance.is_zero() {
        storage.delete(key);
    } else {
        storage.put(key, new_balance.into_byte_string());
    }
    true
}
