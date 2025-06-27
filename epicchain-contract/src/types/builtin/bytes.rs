// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[cfg(target_family = "wasm")]
use crate::types::placeholder::*;

/// Represents a byte array.
#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Bytes(Placeholder);

#[cfg(not(target_family = "wasm"))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    /// Creates a new empty byte array.
    #[inline(always)]
    pub fn new() -> Self {
        #[cfg(target_family = "wasm")]
        {
            Self(Placeholder::new(0))
        }
        #[cfg(not(target_family = "wasm"))]
        {
            Self(Vec::new())
        }
    }

    /// Creates a byte array from a slice.
    #[inline(always)]
    pub fn from_slice(slice: &[u8]) -> Self {
        #[cfg(target_family = "wasm")]
        {
            // For WASM target, create placeholder with safe conversion
            Self(Placeholder::new(slice.len() as i32))
        }
        #[cfg(not(target_family = "wasm"))]
        {
            Self(slice.to_vec())
        }
    }

    /// Returns the length of the byte array.
    #[inline(always)]
    pub fn len(&self) -> usize {
        #[cfg(target_family = "wasm")]
        {
            // For WASM target, return 0 as placeholder
            0
        }
        #[cfg(not(target_family = "wasm"))]
        {
            self.0.len()
        }
    }

    /// Returns whether the byte array is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        #[cfg(target_family = "wasm")]
        {
            // For WASM target, return true as placeholder
            true
        }
        #[cfg(not(target_family = "wasm"))]
        {
            self.0.is_empty()
        }
    }

    /// Returns a reference to the underlying bytes.
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        #[cfg(target_family = "wasm")]
        {
            // For WASM target, return empty slice as placeholder
            &[]
        }
        #[cfg(not(target_family = "wasm"))]
        {
            &self.0
        }
    }

    /// Returns a reference to the underlying bytes as a slice (alias for as_bytes).
    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(target_family = "wasm")]
crate::impl_placeholder!(Bytes);
