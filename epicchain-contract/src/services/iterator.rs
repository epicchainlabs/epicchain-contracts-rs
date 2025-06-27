// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[cfg(target_family = "wasm")]
use crate::{storage::Iter as StorageIterator};

#[cfg(not(target_family = "wasm"))]
use crate::{types::Any, storage::Iter as StorageIterator};

/// Provides functionality for iterating over collections.
pub struct Iterator;

#[cfg(not(target_family = "wasm"))]
impl Iterator {
    /// Advances the iterator to the next element.
    #[inline(always)]
    pub fn next<T>(_iterator: StorageIterator<T>) -> bool {
        unimplemented!("This function is only available in WASM target")
    }

    /// Gets the current element in the iterator.
    #[inline(always)]
    pub fn value<T>(_iterator: StorageIterator<T>) -> Any {
        unimplemented!("This function is only available in WASM target")
    }
}

#[cfg(target_family = "wasm")]
impl Iterator {
    /// Advances the iterator to the next element.
    #[inline(always)]
    pub fn next<T: crate::types::placeholder::FromPlaceholder>(mut iterator: StorageIterator<T>) -> bool {
        iterator.next()
    }

    /// Gets the current element in the iterator.
    #[inline(always)]
    pub fn value<T: crate::types::placeholder::FromPlaceholder>(iterator: StorageIterator<T>) -> T {
        iterator.value()
    }
}
