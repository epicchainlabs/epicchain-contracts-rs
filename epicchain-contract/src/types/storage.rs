// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::types::{placeholder::*, *};
use crate::serialize::{EpicChainSerializable, serialize_to_bytestring, deserialize_from_bytestring};

/// Represents a storage context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypesStorageContext(pub u32);

impl TypesStorageContext {
    /// Convert to storage::StorageContext
    pub fn to_storage_context(&self) -> crate::storage::StorageContext {
        #[cfg(target_family = "wasm")]
        {
            // In WASM, we need to get the actual storage context
            crate::storage::StorageContext::new()
        }

        #[cfg(not(target_family = "wasm"))]
        {
            // In non-WASM, we can create a placeholder with the same value
            // Convert u32 to i32 safely
            let value = self.0 as i32;
            crate::storage::StorageContext(crate::types::placeholder::Placeholder::new(value))
        }
    }
}

/// Represents a read-only storage context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadOnlyStorageContext(pub u32);

/// Represents a storage iterator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorageIterator(pub u32);

/// Represents the options for finding storage items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindOptions {
    /// Find all items.
    All = 0,
    /// Find only keys.
    KeysOnly = 1,
    /// Find only values.
    ValuesOnly = 2,
    /// Find items in descending order.
    Descending = 4,
    /// Find items in descending order and only keys.
    DescendingKeysOnly = 5,
    /// Find items in descending order and only values.
    DescendingValuesOnly = 6,
}

/// Represents a storage item.
#[derive(Debug)]
pub struct StorageItem<T> {
    context: TypesStorageContext,
    key: ByteString,
    _marker: core::marker::PhantomData<T>,
}

impl<T> StorageItem<T>
where
    T: EpicChainSerializable,
{
    /// Creates a new storage item.
    #[inline(always)]
    pub fn new(context: TypesStorageContext, key: impl Into<ByteString>) -> Self {
        Self {
            context,
            key: key.into(),
            _marker: core::marker::PhantomData,
        }
    }

    /// Gets the value of the storage item.
    #[inline(always)]
    pub fn get(&self) -> Option<T> {
        let value = crate::services::storage::Storage::get(self.context.to_storage_context(), self.key.clone());
        value.and_then(|v| {
            match deserialize_from_bytestring(&v) {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        })
    }

    /// Puts a value into the storage item.
    #[inline(always)]
    pub fn put(&self, value: T) {
        match serialize_to_bytestring(&value) {
            Ok(serialized) => {
                crate::services::storage::Storage::put(self.context.to_storage_context(), self.key.clone(), serialized);
            }
            Err(_) => {
                // Handle serialization error - for now, do nothing
                // In a production system, this should be logged or handled appropriately
            }
        }
    }

    /// Deletes the storage item.
    #[inline(always)]
    pub fn delete(&self) {
        crate::services::storage::Storage::delete(self.context.to_storage_context(), self.key.clone());
    }
}

/// Represents a storage map.
#[derive(Debug)]
pub struct StorageMap<K, V> {
    context: TypesStorageContext,
    prefix: ByteString,
    _marker_k: core::marker::PhantomData<K>,
    _marker_v: core::marker::PhantomData<V>,
}

impl<K, V> StorageMap<K, V>
where
    K: EpicChainSerializable,
    V: EpicChainSerializable,
{
    /// Creates a new storage map.
    #[inline(always)]
    pub fn new(context: TypesStorageContext, prefix: impl Into<ByteString>) -> Self {
        Self {
            context,
            prefix: prefix.into(),
            _marker_k: core::marker::PhantomData,
            _marker_v: core::marker::PhantomData,
        }
    }

    /// Gets the key for the given key.
    #[inline(always)]
    fn get_key(&self, key: &K) -> ByteString {
        match serialize_to_bytestring(key) {
            Ok(serialized) => {
                let mut result = self.prefix.clone();
                result.extend(serialized);
                result
            }
            Err(_) => {
                // If serialization fails, return just the prefix
                // In a production system, this should be handled more gracefully
                self.prefix.clone()
            }
        }
    }

    /// Gets the value for the given key.
    #[inline(always)]
    pub fn get(&self, key: K) -> Option<V> {
        let storage_key = self.get_key(&key);
        let value = crate::services::storage::Storage::get(self.context.to_storage_context(), storage_key);
        value.and_then(|v| {
            match deserialize_from_bytestring(&v) {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        })
    }

    /// Puts a value into the storage map.
    #[inline(always)]
    pub fn put(&self, key: K, value: V) {
        let storage_key = self.get_key(&key);
        match serialize_to_bytestring(&value) {
            Ok(serialized) => {
                crate::services::storage::Storage::put(self.context.to_storage_context(), storage_key, serialized);
            }
            Err(_) => {
                // Handle serialization error - for now, do nothing
                // In a production system, this should be logged or handled appropriately
            }
        }
    }

    /// Deletes the value for the given key.
    #[inline(always)]
    pub fn delete(&self, key: K) {
        let storage_key = self.get_key(&key);
        crate::services::storage::Storage::delete(self.context.to_storage_context(), storage_key);
    }
}
