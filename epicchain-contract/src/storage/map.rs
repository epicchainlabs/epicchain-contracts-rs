// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{
    env,
    storage::StorageContext,
    types::{
        builtin::{
            string::ByteString,
            nullable::Nullable,
        },
        placeholder::{Placeholder, FromPlaceholder},
    },
};

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct StorageMap {
    cx: StorageContext,
}

#[cfg(not(target_family = "wasm"))]
pub struct StorageMap {
    items: std::collections::BTreeMap<Vec<u8>, Vec<u8>>,
}

#[cfg(target_family = "wasm")]
impl StorageMap {
    #[inline(always)]
    pub fn new() -> Self {
        Self { cx: StorageContext::new() }
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn get(&self, key: ByteString) -> Nullable<ByteString> {
        let value = unsafe { env::syscall::system_storage_string_key_get(self.cx.clone(), key) };
        Nullable::new(ByteString::from_placeholder(value))
    }

    #[inline(always)]
    #[cfg(target_family = "wasm")]
    #[rustfmt::skip]
    pub fn put(&mut self, key: ByteString, value: ByteString) {
        unsafe { env::syscall::system_storage_string_key_put(self.cx.clone(), key, value) }
    }

    #[inline(always)]
    #[cfg(target_family = "wasm")]
    #[rustfmt::skip]
    pub fn delete(&mut self, key: ByteString) {
        unsafe { env::syscall::system_storage_string_key_delete(self.cx.clone(), key) }
    }
}

#[cfg(not(target_family = "wasm"))]
impl StorageMap {
    pub fn new() -> Self {
        Self { items: std::collections::BTreeMap::new() }
    }

    pub fn get(&self, key: ByteString) -> Nullable<ByteString> {
        self.items
            .get(key.as_bytes())
            .map(|value| ByteString::with_bytes(value))
            .map(Nullable::new)
            .unwrap_or_default()
    }

    pub fn put(&mut self, key: ByteString, value: ByteString) {
        self.items.insert(key.as_bytes().to_vec(), value.as_bytes().to_vec());
    }

    pub fn delete(&mut self, key: ByteString) {
        self.items.remove(key.as_bytes());
    }
}
