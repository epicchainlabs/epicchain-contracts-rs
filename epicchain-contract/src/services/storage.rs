// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[cfg(target_family = "wasm")]
use crate::{env, types::{ByteString, Bytes, FindOptions, placeholder::FromPlaceholder}, storage::{StorageContext, ReadOnlyStorageContext, Iter as StorageIterator}};

#[cfg(not(target_family = "wasm"))]
use crate::{types::{ByteString, Bytes, FindOptions}, storage::{StorageContext, ReadOnlyStorageContext, Iter as StorageIterator}};

/// Provides functionality for reading and writing to persistent storage.
pub struct Storage;

#[cfg(not(target_family = "wasm"))]
impl Storage {
    /// Gets the storage context for the current contract.
    #[inline(always)]
    pub fn get_context() -> StorageContext {
        // For non-WASM targets (tests), return a mock context
        StorageContext::new()
    }

    /// Gets the read-only storage context for the current contract.
    #[inline(always)]
    pub fn get_read_only_context() -> ReadOnlyStorageContext {
        // For non-WASM targets (tests), return a mock read-only context
        ReadOnlyStorageContext::new()
    }

    /// Converts a storage context to a read-only storage context.
    #[inline(always)]
    pub fn as_read_only(context: StorageContext) -> ReadOnlyStorageContext {
        // For non-WASM targets (tests), convert to read-only context
        context.as_readonly()
    }

    /// Gets the value corresponding to the given key from storage.
    #[inline(always)]
    pub fn get(_context: StorageContext, _key: ByteString) -> Option<ByteString> {
        // For non-WASM targets (tests), return None (no persistent storage)
        None
    }

    /// Gets the value corresponding to the given key from storage.
    #[inline(always)]
    pub fn get_with_bytes_key(_context: StorageContext, _key: Bytes) -> Option<ByteString> {
        // For non-WASM targets (tests), return None (no persistent storage)
        None
    }

    /// Puts the key-value pair into storage.
    #[inline(always)]
    pub fn put(_context: StorageContext, _key: ByteString, _value: ByteString) {
        // For non-WASM targets (tests), do nothing (no persistent storage)
    }

    /// Puts the key-value pair into storage.
    #[inline(always)]
    pub fn put_with_bytes_key(_context: StorageContext, _key: Bytes, _value: ByteString) {
        // For non-WASM targets (tests), do nothing (no persistent storage)
    }

    /// Deletes the key-value pair from storage.
    #[inline(always)]
    pub fn delete(_context: StorageContext, _key: ByteString) {
        // For non-WASM targets (tests), do nothing (no persistent storage)
    }

    /// Deletes the key-value pair from storage.
    #[inline(always)]
    pub fn delete_with_bytes_key(_context: StorageContext, _key: Bytes) {
        // For non-WASM targets (tests), do nothing (no persistent storage)
    }

    /// Finds the key-value pairs in storage that match the given prefix.
    #[inline(always)]
    pub fn find<T>(_context: StorageContext, _prefix: ByteString, _options: FindOptions) -> StorageIterator<T> {
        // For non-WASM targets (tests), return empty iterator
        StorageIterator::new()
    }

    /// Finds the key-value pairs in storage that match the given prefix.
    #[inline(always)]
    pub fn find_with_bytes_key<T>(_context: StorageContext, _prefix: Bytes, _options: FindOptions) -> StorageIterator<T> {
        // For non-WASM targets (tests), return empty iterator
        StorageIterator::new()
    }
}

#[cfg(target_family = "wasm")]
impl Storage {
    /// Gets the storage context for the current contract.
    #[inline(always)]
    pub fn get_context() -> StorageContext {
        unsafe { env::syscall::system_storage_get_context() }
    }

    /// Gets the read-only storage context for the current contract.
    #[inline(always)]
    pub fn get_read_only_context() -> ReadOnlyStorageContext {
        unsafe { env::syscall::system_storage_get_readonly_context() }
    }

    /// Converts a storage context to a read-only storage context.
    #[inline(always)]
    pub fn as_read_only(context: StorageContext) -> ReadOnlyStorageContext {
        unsafe { env::syscall::system_storage_as_readonly(context) }
    }

    /// Gets the value corresponding to the given key from storage.
    #[inline(always)]
    pub fn get(context: StorageContext, key: ByteString) -> Option<ByteString> {
        let result = unsafe { env::syscall::system_storage_string_key_get(context, key) };
        if result.is_null() {
            None
        } else {
            Some(ByteString::from_placeholder(result))
        }
    }

    /// Gets the value corresponding to the given key from storage.
    #[inline(always)]
    pub fn get_with_bytes_key(context: StorageContext, key: Bytes) -> Option<ByteString> {
        let result = unsafe { env::syscall::system_storage_bytes_key_get(context, key) };
        if result.is_null() {
            None
        } else {
            Some(ByteString::from_placeholder(result))
        }
    }

    /// Puts the key-value pair into storage.
    #[inline(always)]
    pub fn put(context: StorageContext, key: ByteString, value: ByteString) {
        unsafe { env::syscall::system_storage_string_key_put(context, key, value) }
    }

    /// Puts the key-value pair into storage.
    #[inline(always)]
    pub fn put_with_bytes_key(context: StorageContext, key: Bytes, value: ByteString) {
        unsafe { env::syscall::system_storage_bytes_key_put(context, key, value) }
    }

    /// Deletes the key-value pair from storage.
    #[inline(always)]
    pub fn delete(context: StorageContext, key: ByteString) {
        unsafe { env::syscall::system_storage_string_key_delete(context, key) }
    }

    /// Deletes the key-value pair from storage.
    #[inline(always)]
    pub fn delete_with_bytes_key(context: StorageContext, key: Bytes) {
        unsafe { env::syscall::system_storage_bytes_key_delete(context, key) }
    }

    /// Finds the key-value pairs in storage that match the given prefix.
    #[inline(always)]
    pub fn find<T>(context: StorageContext, prefix: ByteString, options: FindOptions) -> StorageIterator<T> {
        let iter = unsafe { env::syscall::system_storage_string_key_scan_prefix(context, prefix, options) };
        StorageIterator::from_placeholder(iter)
    }

    /// Finds the key-value pairs in storage that match the given prefix.
    #[inline(always)]
    pub fn find_with_bytes_key<T>(context: StorageContext, prefix: Bytes, options: FindOptions) -> StorageIterator<T> {
        let iter = unsafe { env::syscall::system_storage_bytes_key_scan_prefix(context, prefix, options) };
        StorageIterator::from_placeholder(iter)
    }
}
