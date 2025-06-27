// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::{placeholder::*, *}};

/// ByteString is a non utf-8 string
#[cfg(not(target_family = "wasm"))]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteString(Vec<u8>);

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct ByteString(Placeholder);

#[cfg(target_family = "wasm")]
impl ByteString {
    #[inline(always)]
    pub fn empty() -> Self {
        unsafe { env::asm::string_empty() }
    }

    /// Creates a ByteString from a byte slice (WASM version)
    /// Note: For WASM target, this creates from literal for now
    /// In a full implementation, this would use proper byte conversion
    #[inline(always)]
    pub fn from_bytes(_bytes: &[u8]) -> Self {
        // For WASM target, we'll need to implement this properly
        // For now, return empty string as placeholder
        Self::empty()
    }

    /// Returns the underlying bytes as a vector (WASM version)
    /// Note: For WASM target, this is a placeholder implementation
    /// In a full implementation, this would extract actual bytes
    #[inline(always)]
    pub fn to_bytes(&self) -> Vec<u8> {
        // For WASM target, we'll need to implement this properly
        // For now, return empty vector as placeholder
        vec![]
    }

    /// Returns the underlying bytes as a slice (WASM version)
    /// Note: For WASM target, this is a placeholder implementation
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        // For WASM target, we'll need to implement this properly
        // For now, return empty slice as placeholder
        &[]
    }

    /// Creates a ByteString from a byte slice (WASM version)
    #[inline(always)]
    pub fn from_slice(bytes: &[u8]) -> Self {
        Self::from_bytes(bytes)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        unsafe { env::asm::string_len(Self(self.0)) }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub fn substr(&self, start_index: usize, count: usize) -> Self {
        unsafe { env::asm::string_sub(Self(self.0), start_index, start_index + count) }
    }

    #[inline(always)]
    pub fn concat(&self, other: &Self) -> Self {
        unsafe { env::asm::string_concat(Self(self.0), Self(other.0)) }
    }

    #[inline(always)]
    pub fn hex_encode(&self) -> Self {
        unsafe { env::stdlib::hex_encode(Self(self.0)) }
    }

    #[inline(always)]
    pub fn from_literal(literal: &str) -> Self {
        unsafe { env::asm::string_from_literal(literal) }
    }

    /// Extend this ByteString with another ByteString (WASM version)
    #[inline(always)]
    pub fn extend(&mut self, other: ByteString) {
        *self = self.concat(&other);
    }
}

#[cfg(not(target_family = "wasm"))]
impl ByteString {
    pub fn new(value: impl Into<Vec<u8>>) -> Self {
        Self(value.into())
    }

    pub fn empty() -> Self {
        Self(vec![])
    }

    pub(crate) fn with_bytes(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    /// Creates a ByteString from a byte slice
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    /// Creates a ByteString from a byte slice (alias for from_bytes)
    pub fn from_slice(bytes: &[u8]) -> Self {
        Self::from_bytes(bytes)
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns the underlying bytes as a vector
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn substr(&self, start_index: usize, count: usize) -> Self {
        Self(self.0[start_index..start_index + count].to_vec())
    }

    pub fn concat(&self, other: &Self) -> Self {
        let mut vec = self.0.clone();
        vec.extend(other.0.clone());
        Self(vec)
    }

    pub fn hex_encode(&self) -> Self {
        Self(hex::encode(self.0.as_slice()).into_bytes())
    }

    pub(crate) fn to_string(self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }

    pub fn from_literal(literal: &str) -> Self {
        Self(literal.as_bytes().to_vec())
    }

    pub fn extend(&mut self, other: ByteString) {
        self.0.extend(other.0);
    }
}

impl Default for ByteString {
    #[inline(always)]
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(target_family = "wasm")]
impl PartialEq for ByteString {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        unsafe { env::asm::string_eq(Self(self.0), Self(other.0)) }
    }
}

#[cfg(target_family = "wasm")]
impl Eq for ByteString {}

#[cfg(target_family = "wasm")]
impl Clone for ByteString {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(target_family = "wasm")]
impl core::fmt::Debug for ByteString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ByteString(placeholder)")
    }
}

#[cfg(target_family = "wasm")]
impl PartialOrd for ByteString {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(target_family = "wasm")]
impl Ord for ByteString {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare the actual string contents by comparing their bytes
        let self_bytes = self.as_bytes();
        let other_bytes = other.as_bytes();

        // Compare byte by byte
        let len = std::cmp::min(self_bytes.len(), other_bytes.len());
        for i in 0..len {
            let self_byte = self_bytes[i];
            let other_byte = other_bytes[i];
            if self_byte != other_byte {
                return self_byte.cmp(&other_byte);
            }
        }

        // If all bytes are equal up to the minimum length, compare lengths
        self_bytes.len().cmp(&other_bytes.len())
    }
}

#[cfg(target_family = "wasm")]
impl FromPlaceholder for ByteString {
    #[inline(always)]
    fn from_placeholder(placeholder: Placeholder) -> Self {
        Self(placeholder)
    }
}

#[cfg(target_family = "wasm")]
impl IntoPlaceholder for ByteString {
    #[inline(always)]
    fn into_placeholder(self) -> Placeholder {
        self.0
    }
}

/// convert the type as a ByteString
/// like reinterpret cast
pub trait IntoByteString {
    fn into_byte_string(self) -> ByteString;
}

pub trait FromByteString {
    fn from_byte_string(src: ByteString) -> Self;
}

// Implement Primitive trait for ByteString
impl crate::types::builtin::primitive::Primitive for ByteString {}
